use crate::Error;
pub use rustls::{ServerConfig, ServerConnection};
use std::{net::TcpStream, path::Path, sync::Arc};

/// A read/write enabled stream that implements the server's portion of tls
pub type ServerStream = rustls::StreamOwned<rustls::ServerConnection, std::net::TcpStream>;

/// This function creates a tls configuration object with no client auth.
///
/// # Arguments
///
/// - `cert_file` is the path to your certification chain file.
/// - `private_key_file` is the path to your private key file.
///
/// # Returns
///
/// A [`ServerConfig`]  wrapped in [`Arc`].
///
/// The Arc is needed as you probably want to handle multiple connections at the same time.
///
/// # Errors
///
/// - If the key files cannot be parsed, you will receive an `Err(ez_tls::Error::PemError(e))` with the underlying error object inside.
/// - If the certification is invalid, you will receive an `Err(ez_tls::Error::TlsError(e))` with the underlying error object inside.
pub fn setup_tls_config(
    cert_file: &Path,
    private_key_file: &Path,
) -> Result<Arc<ServerConfig>, Error> {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
    let cert_chain = CertificateDer::pem_file_iter(cert_file)?
        .filter_map(Result::ok)
        .collect();
    let key_der = PrivateKeyDer::from_pem_file(private_key_file)?;
    Ok(Arc::new(
        ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key_der)?,
    ))
}

fn setup_tls_connection(config: &Arc<ServerConfig>) -> Result<ServerConnection, Error> {
    ServerConnection::new(Arc::clone(config)).map_err(Error::TlsError)
}

fn setup_tls_stream(conn: ServerConnection, sock: TcpStream) -> ServerStream {
    ServerStream::new(conn, sock)
}

/// Wraps up your [`ServerConnection`] and [`TcpStream`] into a handy [`ServerStream`]
///
/// # Returns
///
/// [`ServerStream`]
///
/// # Errors
///
/// - If there is a problem with TLS during the handshake, or if the [`TcpStream`] can't be written to, you'll recieve an `Err(ez_tls::Error::TlsError(e)` with the underlying error inside.
pub fn setup_tls(config: &Arc<ServerConfig>, sock: TcpStream) -> Result<ServerStream, Error> {
    Ok(setup_tls_stream(
        setup_tls_connection(&Arc::clone(config))?,
        sock,
    ))
}
