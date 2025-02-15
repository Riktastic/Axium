use axum::{
    response::IntoResponse, 
    Json, 
    extract::State
};
use serde_json::json;
use sqlx::PgPool;
use sysinfo::{System, RefreshKind, Disks};
use tokio::{task, join};
use std::sync::{Arc, Mutex};
use tracing::instrument; // For logging
use crate::models::health::HealthResponse;

// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Successfully fetched health status", body = HealthResponse),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument(skip(database_connection))]
pub async fn get_health(State(database_connection): State<PgPool>) -> impl IntoResponse {
    // Use Arc and Mutex to allow sharing System between tasks
    let system = Arc::new(Mutex::new(System::new_with_specifics(RefreshKind::everything())));

    // Run checks in parallel
    let (cpu_result, mem_result, disk_result, process_result, db_result, net_result) = join!(
        task::spawn_blocking({
            let system = Arc::clone(&system);
            move || {
                let mut system = system.lock().unwrap();  // Lock the mutex and get a mutable reference
                check_cpu_usage(&mut system)  // Pass the mutable reference
            }
        }),
        task::spawn_blocking({
            let system = Arc::clone(&system);
            move || {
                let mut system = system.lock().unwrap();  // Lock the mutex and get a mutable reference
                check_memory(&mut system)  // Pass the mutable reference
            }
        }),
        task::spawn_blocking({
            move || {
                check_disk_usage()  // Does not need a system reference.
            }
        }),
        task::spawn_blocking({
            let system = Arc::clone(&system);
            move || {
                let mut system = system.lock().unwrap();  // Lock the mutex and get a mutable reference
                check_processes(&mut system, &["postgres", "Code"])  // Pass the mutable reference
            }
        }),
        check_database_connection(&database_connection), // Async function
        task::spawn_blocking(check_network_connection) // Blocking, okay in spawn_blocking
    );

    let mut status = "healthy";
    let mut details = json!({});

    // Process CPU result
    if let Ok(Ok(cpu_details)) = cpu_result {
        details["cpu_usage"] = json!(cpu_details);
        if cpu_details["status"] == "low" {
            status = "degraded";
        }
    } else {
        details["cpu_usage"] = json!({ "status": "error", "message": "Failed to retrieve CPU usage" });
        status = "degraded";
    }

    // Process Memory result
    if let Ok(Ok(mem_details)) = mem_result {
        details["memory"] = json!(mem_details);
        if mem_details["status"] == "low" {
            status = "degraded";
        }
    } else {
        details["memory"] = json!({ "status": "error", "message": "Failed to retrieve memory information" });
        status = "degraded";
    }

    // Process Disk result
    if let Ok(Ok(disk_details)) = disk_result {
        details["disk_usage"] = json!(disk_details);
        if disk_details["status"] == "critical" {
            status = "degraded";
        }
    } else {
        details["disk_usage"] = json!({ "status": "error", "message": "Failed to retrieve disk usage" });
        status = "degraded";
    }

    // Process Process result
    if let Ok(Ok(process_details)) = process_result {
        details["important_processes"] = json!(process_details);
        if process_details.iter().any(|p| p["status"] == "not running") {
            status = "degraded";
        }
    } else {
        details["important_processes"] = json!({ "status": "error", "message": "Failed to retrieve process information" });
        status = "degraded";
    }

    // Process Database result
    if let Ok(db_status) = db_result {
        details["database"] = json!({ "status": if db_status { "ok" } else { "degraded" } });
        if !db_status {
            status = "degraded";
        }
    } else {
        details["database"] = json!({ "status": "error", "message": "Failed to retrieve database status" });
        status = "degraded";
    }

    // Process Network result
    if let Ok(Ok(net_status)) = net_result {
        details["network"] = json!({ "status": if net_status { "ok" } else { "degraded" } });
        if !net_status {
            status = "degraded";
        }
    } else {
        details["network"] = json!({ "status": "error", "message": "Failed to retrieve network status" });
        status = "degraded";
    }

    Json(json!({
        "status": status,
        "details": details,
    }))
}

// Helper functions

#[instrument]
fn check_cpu_usage(system: &mut System) -> Result<serde_json::Value, ()> {
    system.refresh_cpu_usage();
    let usage = system.global_cpu_usage();
    let available = 100.0 - usage;
    Ok(json!( {
        "usage_percentage": format!("{:.2}", usage),
        "available_percentage": format!("{:.2}", available),
        "status": if available < 10.0 { "low" } else { "normal" },
    }))
}

#[instrument]
fn check_memory(system: &mut System) -> Result<serde_json::Value, ()> {
    system.refresh_memory();
    let available = system.available_memory() / 1024 / 1024; // Convert to MB
    Ok(json!( {
        "available_mb": available,
        "status": if available < 512 { "low" } else { "normal" },
    }))
}

#[instrument]
fn check_disk_usage() -> Result<serde_json::Value, ()> {
    // Create a new Disks object and refresh the disk information
    let mut disks = Disks::new();
    disks.refresh(false); // Refresh disk information without performing a full refresh

    // Iterate through the list of disks and check the usage for each one
    let usage: Vec<_> = disks.list().iter().map(|disk| {
        let total = disk.total_space() as f64;
        let available = disk.available_space() as f64;
        let used_percentage = ((total - available) / total) * 100.0;
        used_percentage
    }).collect();

    // Get the maximum usage percentage
    let max_usage = usage.into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0);

    // Return the result as a JSON object
    Ok(json!( {
        "used_percentage": format!("{:.2}", max_usage),
        "status": if max_usage > 90.0 { "critical" } else { "ok" },
    }))
}

#[instrument]
fn check_processes(system: &mut System, processes: &[&str]) -> Result<Vec<serde_json::Value>, ()> {
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    
    let process_statuses: Vec<_> = processes.iter().map(|&name| {
        // Adjust process names based on the platform and check if they are running
        let adjusted_name = if cfg!(target_os = "windows") {
            match name {
                "postgres" => "postgres.exe",  // Postgres on Windows
                "Code" => "Code.exe",          // Visual Studio Code on Windows
                _ => name,                     // For other platforms, use the name as is
            }
        } else {
            name  // For non-Windows platforms, use the name as is
        };

        // Check if the translated (adjusted) process is running
        let is_running = system.processes().iter().any(|(_, proc)| proc.name() == adjusted_name);

        // Return a JSON object for each process with its status
        json!({
            "name": name,
            "status": if is_running { "running" } else { "not running" }
        })
    }).collect();

    Ok(process_statuses)
}

async fn check_database_connection(pool: &PgPool) -> Result<bool, sqlx::Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await.map(|_| true).or_else(|_| Ok(false))
}

fn check_network_connection() -> Result<bool, ()> {
    Ok(std::net::TcpStream::connect("8.8.8.8:53").is_ok())
}
