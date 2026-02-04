//! A simple wrapper around [`rustls`] with no-nonsense defaults.
//!
//! If you need more advanced configuration, consider using rustls directly (it's already very good, I just wanted to write a server in like 2 lines of code).

pub use rustls::Error as TlsError;
pub use rustls::pki_types::InvalidDnsNameError as DnsError;
pub use rustls::pki_types::pem::Error as PemError;
pub use std::io::Error as IoError;

/// Helper functions and re-exports for your server
pub mod server;

/// Helper functions and re-exports for your client
pub mod client;

/// A unified error type
#[derive(Debug)]
pub enum Error {
    /// An error related to tls communication
    TlsError(TlsError),
    /// An error related to pem files
    PemError(PemError),
    /// An error related to DNS resolution
    DnsError(DnsError),
    /// An error related to an IO stream
    IoError(IoError),
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TlsError(error) => error.fmt(f),
            Error::PemError(error) => error.fmt(f),
            Error::DnsError(error) => error.fmt(f),
            Error::IoError(error) => error.fmt(f),
        }
    }
}

impl From<TlsError> for Error {
    fn from(value: TlsError) -> Self {
        Self::TlsError(value)
    }
}

impl From<PemError> for Error {
    fn from(value: PemError) -> Self {
        Self::PemError(value)
    }
}

impl From<DnsError> for Error {
    fn from(value: DnsError) -> Self {
        Self::DnsError(value)
    }
}

impl From<IoError> for Error {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}
