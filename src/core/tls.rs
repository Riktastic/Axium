// Standard library imports
use std::{ 
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    io::BufReader,
    fs::File,
    iter,
};

// External crate imports
use axum::serve::Listener;
use rustls::{self, server::ServerConfig, pki_types::{PrivateKeyDer, CertificateDer}};
use rustls_pemfile::{Item, read_one, certs};
use tokio::io::{AsyncRead, AsyncWrite};
use tracing;

// Local crate imports
use crate::config; // Import env config helper

// Function to load TLS configuration from files
pub fn load_tls_config() -> ServerConfig {
    // Get certificate and key file paths from the environment
    let cert_path = config::get_env("SERVER_HTTPS_CERT_FILE_PATH");
    let key_path = config::get_env("SERVER_HTTPS_KEY_FILE_PATH");

    // Open the certificate and key files
    let cert_file = File::open(cert_path).expect("❌ Failed to open certificate file.");
    let key_file = File::open(key_path).expect("❌ Failed to open private key file.");

    // Read the certificate chain and private key from the files
    let mut cert_reader = BufReader::new(cert_file);
    let mut key_reader = BufReader::new(key_file);

    // Read and parse the certificate chain
    let cert_chain: Vec<CertificateDer> = certs(&mut cert_reader)
        .map(|cert| cert.expect("❌ Failed to read certificate."))
        .map(CertificateDer::from)
        .collect();

    // Ensure certificates are found
    if cert_chain.is_empty() {
        panic!("❌ No valid certificates found.");
    }

    // Read the private key from the file
    let key = iter::from_fn(|| read_one(&mut key_reader).transpose())
        .find_map(|item| match item.unwrap() {
            Item::Pkcs1Key(key) => Some(PrivateKeyDer::from(key)),
            Item::Pkcs8Key(key) => Some(PrivateKeyDer::from(key)),
            Item::Sec1Key(key) => Some(PrivateKeyDer::from(key)),
            _ => None,
        })
        .expect("❌  Failed to read a valid private key.");

    // Build and return the TLS server configuration
    ServerConfig::builder()
        .with_no_client_auth()  // No client authentication
        .with_single_cert(cert_chain, key)  // Use the provided cert and key
        .expect("❌  Failed to create TLS configuration.")
}

// Custom listener that implements axum::serve::Listener
#[derive(Clone)]
pub struct TlsListener {
    pub inner: Arc<tokio::net::TcpListener>,  // Inner TCP listener
    pub acceptor: tokio_rustls::TlsAcceptor,  // TLS acceptor for handling TLS handshakes
}

impl Listener for TlsListener {
    type Io = TlsStreamWrapper;  // Type of I/O stream
    type Addr = SocketAddr;  // Type of address (Socket address)

    // Method to accept incoming connections and establish a TLS handshake
    fn accept(&mut self) -> impl Future<Output = (Self::Io, Self::Addr)> + Send {
        let acceptor = self.acceptor.clone();  // Clone the acceptor for async use
        
        async move {
            loop {
                // Accept a TCP connection
                let (stream, addr) = match self.inner.accept().await {
                    Ok((stream, addr)) => (stream, addr),
                    Err(e) => {
                        tracing::error!("❌ Error accepting TCP connection: {}", e);
                        continue;  // Retry on error
                    }
                };

                // Perform TLS handshake
                match acceptor.accept(stream).await {
                    Ok(tls_stream) => {
                        tracing::info!("Successful TLS handshake with {}.", addr);
                        return (TlsStreamWrapper(tls_stream), addr);  // Return TLS stream and address
                    },
                    Err(e) => {
                        tracing::warn!("TLS handshake failed: {} (Client may not trust certificate).", e);
                        continue;  // Retry on error
                    }
                }
            }
        }
    }

    // Method to retrieve the local address of the listener
    fn local_addr(&self) -> std::io::Result<Self::Addr> {
        self.inner.local_addr()
    }
}

// Wrapper for a TLS stream, implementing AsyncRead and AsyncWrite
#[derive(Debug)]
pub struct TlsStreamWrapper(tokio_rustls::server::TlsStream<tokio::net::TcpStream>);

impl AsyncRead for TlsStreamWrapper {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_read(cx, buf)  // Delegate read operation to the underlying TLS stream
    }
}

impl AsyncWrite for TlsStreamWrapper {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)  // Delegate write operation to the underlying TLS stream
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)  // Flush operation for the TLS stream
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_shutdown(cx)  // Shutdown operation for the TLS stream
    }
}

// Allow the TLS stream wrapper to be used in non-blocking contexts (needed for async operations)
impl Unpin for TlsStreamWrapper {}
