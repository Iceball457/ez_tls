A super simple wrapper around [rustls](https://crates.io/crates/rustls)

You only really need three functions to get started:

```rust
ez_tls::server::setup_tls_config(cert_file: &Path, private_key_file: &Path) -> Result<Arc<ServerConfig>, ez_tls::Error>;
ez_tls::server::setup_tls(config: &Arc<ServerConfig>, sock: TcpStream) -> Result<ServerStream>, ez_tls::Error>;

ez_tls::client::setup_tls(host: &str, port: u16, test_cert_file: Option<&Path>) -> Result<ClientStream, ez_tls::Error>;
```

```rust
// server.rs

// Read arguments
let args = std::env::args();
_ = args.next(); // "server.x86_64"
let cert_file: std::path::PathBuf = args.next().into(); // "./test.crt"
let private_key_file: std::path::PathBuf = args.next().into(); // "./test.key"

// Setup Tls config
let config: ez_tls::ServerConfig = ez_tls::server::setup_tls_config(&cert_file, &private_key_file).unwrap();

// Bind to a port and loop over incoming connections
let listener: std::net::TcpListener = std::net::TcpListener::bind("localhost:8000");
for sock in listener.incoming() {
    if let Ok(sock) = sock {
        // You can use this stream object just like you would use any other!
        // (It implements read, too)
        let mut stream: ez_tls::ServerStream = ez_tls::server::setup_tls(&config, sock).unwrap();
        _ = stream.write_all(b"Hello World!");
        stream.conn.send_close_notify();
        _ = stream.flush();
    }
}
```

```rust
// client.rs

// Read arguments
let args = std::env::args();
_ = args.next(); // "client.x86_64"
let host: String = args.next(); // localhost
let port: u16 = args.next().parse(); // 8000
let test_cert_file: PathBuf = args.next().into(); // "./test.crt"

// Connect to server
// Use None for your test cert file in production code! A list of trusted hosts is already included from webpki
let stream: ClientStream = ez_tls::client::setup_tls(host, port, Some(test_cert_file));

// You can use this stream object just like you can any other!
// (It implements write, too)
let mut output: Vec<u8> = vec![];
stream.read_to_end(&mut output);
println!("{}", String::from_utf8_lossy(output.as_slice()));
```
