use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Represents the overall health status of the system.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    /// CPU usage information.
    pub cpu_usage: CpuUsage,
    /// Database status information.
    pub database: DatabaseStatus,
    /// Disk usage information.
    pub disk_usage: DiskUsage,
    /// Memory status information.
    pub memory: MemoryStatus,
}

/// Represents CPU usage information.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CpuUsage {
    /// Percentage of CPU available, represented as a string.
    #[serde(rename = "available_percentage")]
    pub available_pct: String,
    /// Status of the CPU (e.g., "OK", "Warning", "Critical").
    pub status: String,
}

/// Represents database status information.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatabaseStatus {
    /// Status of the database (e.g., "Connected", "Disconnected").
    pub status: String,
}

/// Represents disk usage information.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DiskUsage {
    /// Status of the disk (e.g., "OK", "Warning", "Critical").
    pub status: String,
    /// Percentage of disk space used, represented as a string.
    #[serde(rename = "used_percentage")]
    pub used_pct: String,
}

/// Represents memory status information.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MemoryStatus {
    /// Amount of available memory in megabytes.
    #[serde(rename = "available_mb")]
    pub available_mb: i64,
    /// Status of the memory (e.g., "OK", "Warning", "Critical").
    pub status: String,
}
