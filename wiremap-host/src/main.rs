// Imports
use std::env;
// use std::io;
// use std::io::{Read, Write};
// use std::net::TcpListener;
use std::net::TcpStream;

// Uses
use ssh2::Session;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "Usage: {} <local-port> <gateway-ip> <username> <password>",
            args[0]
        );
        std::process::exit(1);
    }

    // Extract command line arguments into variables
    let local_port: u16 = args[1].parse().expect("Invalid local port");
    let gateway_ip = &args[2];
    let username = &args[3];
    let password = &args[4];

    // Connect to the remote SSH server (the gateway)
    println!("Attempting to create an SSH session into {gateway_ip}...");
    let tcp: TcpStream = TcpStream::connect(format!("{}:22",gateway_ip))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    println!("Session sucessfully created!");

    // Attempt authentication
    println!("Attempting to authenticate...");
    sess.userauth_password(username, password)?;
    assert!(sess.authenticated());
    println!("Authentication successful!");

    return Ok(());
}
