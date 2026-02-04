use crate::Error;
pub use rustls::{ClientConfig, ClientConnection};
use std::{path::Path, sync::Arc};

/// A read/write enabled stream that implements the client's portion of tls
pub type ClientStream = rustls::StreamOwned<rustls::ClientConnection, std::net::TcpStream>;

/// Automatically establishes a tls enabled tcp socket with a given host.
///
/// # Arguments
///
/// - `host` the name of the remote server you wish to connect to. (Remember, for TLS, differences analagous to `localhost`, `127.0.0.1`, and `\[::1\]` are meaningful.)
/// - `port` the remote port you wish to connect to.
/// - `test_cert_file` if you have a local cert file you wish to trust for testing, add it!
///
/// # Returns
///
/// A client stream, which contains a [`ClientConnection`] and a [`TcpStream`].
///
/// # Errors
///
/// This function will fail if the test cert cannot be parsed, or if the tcp connection cannot be made!
pub fn setup_tls(
    host: &str,
    port: u16,
    test_cert_file: Option<&Path>,
) -> Result<ClientStream, Error> {
    use rustls::{
        RootCertStore,
        pki_types::{CertificateDer, pem::PemObject},
    };
    let mut root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
    };
    if let Some(test_cert_file) = test_cert_file {
        root_store.add_parsable_certificates(
            CertificateDer::pem_file_iter(test_cert_file)?.filter_map(std::result::Result::ok),
        );
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let config = Arc::new(config);

    let conn = ClientConnection::new(config, host.to_string().try_into()?)?;
    let sock = std::net::TcpStream::connect((host, port))?;
    Ok(ClientStream::new(conn, sock))
}
