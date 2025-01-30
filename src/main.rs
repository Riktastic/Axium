#[allow(dead_code)]

// Core modules for the configuration, TLS setup, and server creation
mod core;
use core::{config, tls, server}; 
use core::tls::TlsListener;

// Other modules for database, routes, models, and middlewares
mod database;
mod routes;
mod models;
mod middlewares;

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use axum::serve;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();  // Load environment variables from a .env file
    tracing_subscriber::fmt::init();  // Initialize the logging system

    // Print a cool startup message with ASCII art and emojis
    println!("{}", r#"
    Axum-API-Quickstart 
    - An example API built with Rust, Axum, SQLx, and PostgreSQL
    - GitHub: https://github.com/Riktastic/Axum-API-Quickstart/
    "#);

    println!("🚀 Starting Rustapi...");

    // Retrieve server IP and port from the environment, default to 127.0.0.1:3000
    let ip: IpAddr = config::get_env_with_default("SERVER_IP", "127.0.0.1")
        .parse()
        .expect("❌ Invalid IP address format. Please provide a valid IPv4 address. For example 0.0.0.0 or 127.0.0.1.");
    let port: u16 = config::get_env_u16("SERVER_PORT", 3000);
    let socket_addr = SocketAddr::new(ip, port);

    // Create the Axum app instance using the server configuration
    let app = server::create_server().await;

    // Check if HTTPS is enabled in the environment configuration
    if config::get_env_bool("SERVER_HTTPS_ENABLED", false) {
        // If HTTPS is enabled, start the server with secure HTTPS.

        // Bind TCP listener for incoming connections
        let tcp_listener = TcpListener::bind(socket_addr)
            .await
            .expect("❌ Failed to bind to socket. Port might allready be in use."); // Explicit error handling

        // Load the TLS configuration for secure HTTPS connections
        let tls_config = tls::load_tls_config();
        let acceptor = TlsAcceptor::from(Arc::new(tls_config)); // Create a TLS acceptor
        let listener = TlsListener {
            inner: Arc::new(tcp_listener), // Wrap TCP listener in TlsListener
            acceptor: acceptor,
        };

        println!("🔒 Server started with HTTPS at: https://{}:{}", ip, port);

        // Serve the app using the TLS listener (HTTPS)
        serve(listener, app.into_make_service())
            .await
            .expect("❌ Server failed to start with HTTPS. Did you provide valid certificate and key files?");

    } else {
        // If HTTPS is not enabled, start the server with non-secure HTTP.

        // Bind TCP listener for non-secure HTTP connections
        let listener = TcpListener::bind(socket_addr)
            .await
            .expect("❌ Failed to bind to socket. Port might allready be in use."); // Explicit error handling

        println!("🔓 Server started with HTTP at: http://{}:{}", ip, port);

        // Serve the app using the non-secure TCP listener (HTTP)
        serve(listener, app.into_make_service())
            .await
            .expect("❌ Server failed to start without HTTPS.");
    }
}
