#[allow(dead_code)]

// Core modules for the configuration, TLS setup, and server creation
mod core;
use core::{config, server}; 

// Other modules for database, routes, models, and middlewares
mod database;
mod routes;
mod models;
mod middlewares;
mod handlers;
mod utils;

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::signal;

use axum_server::tls_rustls::RustlsConfig;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("❌ Failed to install Ctrl+C handler.");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("\n⏳  Shutdown signal received, starting graceful shutdown.");
}

fn display_additional_info(protocol: &str, ip: IpAddr, port: u16) {
    println!("\n📖  Explore the API using Swagger ({0}://{1}:{2}/swagger)\n    or import the OpenAPI spec ({0}://{1}:{2}/openapi.json).", protocol, ip, port);
    println!("\n🩺  Ensure your Docker setup is reliable,\n    by pointing its healthcheck to {0}://{1}:{2}/health", protocol, ip, port);
    println!("\nPress [CTRL] + [C] to gracefully shutdown.");
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();  // Load environment variables from a .env file

    tracing_subscriber::fmt::init();  // Initialize the logging system

    println!("{}", r#"
<<<<<<< HEAD
           db                      88                                   
          d88b                     ""                                   
         d8'`8b                                                         
        d8'  `8b      8b,     ,d8  88  88       88  88,dPYba,,adPYba,   
       d8YaaaaY8b      `Y8, ,8P'   88  88       88  88P'   "88"    "8a  
      d8""""""""8b       )888(     88  88       88  88      88      88  
     d8'        `8b    ,d8" "8b,   88  "8a,   ,a88  88      88      88  
    d8'          `8b  8P'     `Y8  88   `"YbbdP'Y8  88      88      88  

              - GitHub: https://github.com/Riktastic/Axium
              - Version: 1.0
=======
    Axum-API-Quickstart 
    - An example API built with Rust, Axum, SQLx, and PostgreSQL
    - GitHub: https://github.com/Riktastic/Axum-API-Quickstart/
>>>>>>> 830dbdb2074fc62e056ef70d374bea3f26ac0589
    "#);

    println!("🦖  Starting Axium...");

<<<<<<< HEAD
=======
    // Retrieve server IP and port from the environment, default to 127.0.0.1:3000
>>>>>>> 830dbdb2074fc62e056ef70d374bea3f26ac0589
    let ip: IpAddr = config::get_env_with_default("SERVER_IP", "127.0.0.1")
        .parse()
        .expect("❌  Invalid IP address format.");
    let port: u16 = config::get_env_u16("SERVER_PORT", 3000);
    let addr = SocketAddr::new(ip, port);
    let app = server::create_server().await;

    let is_https = config::get_env_bool("SERVER_HTTPS_ENABLED", false);
    let is_http2 = config::get_env_bool("SERVER_HTTPS_HTTP2_ENABLED", false);
    let protocol = if is_https { "https" } else { "http" };


    if is_https {
        // HTTPS

        // Ensure that the crypto provider is initialized before using rustls
        rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap_or_else(|e| {
            eprintln!("❌ Crypto provider initialization failed: {:?}", e);
            std::process::exit(1);
        });

        // Get certificate and key file paths from environment variables
        let cert_path = config::get_env("SERVER_HTTPS_CERT_FILE_PATH");
        let key_path = config::get_env("SERVER_HTTPS_KEY_FILE_PATH");

        // Set up Rustls config with HTTP/2 support
        let (certs, key) = {
            // Load certificate chain
            let certs = tokio::fs::read(&cert_path)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("❌  Failed to read certificate file: {}", e);
                    std::process::exit(1);
                });
            
            // Load private key
            let key = tokio::fs::read(&key_path)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("❌  Failed to read key file: {}", e);
                    std::process::exit(1);
                });

            // Parse certificates and private key
            let certs = rustls_pemfile::certs(&mut &*certs)
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|e| {
                    eprintln!("❌  Failed to parse certificates: {}", e);
                    std::process::exit(1);
                });

            let mut keys = rustls_pemfile::pkcs8_private_keys(&mut &*key)
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|e| {
                    eprintln!("❌  Failed to parse private key: {}", e);
                    std::process::exit(1);
                });

            let key = keys.remove(0);
    
            // Wrap the private key in the correct type
            let key = rustls::pki_types::PrivateKeyDer::Pkcs8(key);

            (certs, key)
        };

        let mut config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .unwrap_or_else(|e| {
            eprintln!("❌  Failed to build TLS configuration: {}", e);
            std::process::exit(1);
        });

        if is_http2 {
            config.alpn_protocols = vec![b"h2".to_vec()];
        }

        let rustls_config = RustlsConfig::from_config(Arc::new(config));

        println!("🔒  Server started with HTTPS at: {}://{}:{}", protocol, ip, port);

        display_additional_info(protocol, ip, port);

        // Create the server future but don't await it yet
        let server = axum_server::bind_rustls(addr, rustls_config)
            .serve(app.into_make_service());

        tokio::select! {
            result = server => {
                if let Err(e) = result {
                    eprintln!("❌  Server failed to start with HTTPS: {}", e);
                }
            },
            _ = shutdown_signal() => {},
        }
    } else {
        // HTTP

        println!("🔓  Server started with HTTP at: {}://{}:{}", protocol, ip, port);

        display_additional_info(protocol, ip, port);

        // Create the server future but don't await it yet
        let server = axum_server::bind(addr)
            .serve(app.into_make_service());

        tokio::select! {
            result = server => {
                if let Err(e) = result {
                    eprintln!("❌  Server failed to start with HTTP: {}", e);
                }
            },
            _ = shutdown_signal() => {},
        }
    }
    println!("\n✔️   Server has shut down gracefully.");
}