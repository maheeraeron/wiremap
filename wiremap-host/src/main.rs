// Imports
use ssh2::{Channel, Session};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, str};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For verbose logging
    // env::set_var("RUST_BACKTRACE", "full");

    // Extract command line arguments into variables
    let local_port: u16 = 5001;
    let gateway_ip: &str = "dev.wiremap.io";
    let remote_port: u16 = 10004;
    let username: &str = "root";
    let password: &str = "Webma$ter1";

    // Connect to the remote SSH server (the gateway)
    println!("Attempting to create an SSH session into {gateway_ip}...");
    let tcp: TcpStream = TcpStream::connect(format!("{}:22", gateway_ip))?;
    let mut session: Session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    println!("Session sucessfully created!");

    // Attempt authentication
    println!("Attempting to authenticate...");
    session.userauth_password(username, password)?;
    assert!(session.authenticated());
    println!("Authentication successful!");

    // Create a TCP listener on the local port
    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", local_port))?;
    println!("Listening on port {}", local_port);

    // Create channel
    let mut channel: Channel = match session.channel_direct_tcpip("localhost", 10004, None) {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Error opening channel: {:?}", e);
            return Ok(());
        }
    };

    // Listen for TCP connections
    for stream in listener.incoming() {
        println!("===============================================================================");

        // Read the incoming request
        let mut stream = stream.unwrap();
        let mut request = vec![0; 8192];
        let read_bytes = stream.read(&mut request).unwrap();
        println!(
            "REQUEST ({} BYTES):\n{}",
            read_bytes,
            str::from_utf8(&request).unwrap()
        );

        // Write data from buffer into channel
        channel.write_all(&request)?;
        println!("Request sent successfully over SSH!");
    }
    return Ok(());

}