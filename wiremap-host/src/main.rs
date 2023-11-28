// Imports
use std::env;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// Uses
use ssh2::Session;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "Usage: {} <local-port> <gateway-ip> <remote-port> <username> <password>",
            args[0]
        );
        std::process::exit(1);
    }

    // Extract command line arguments into variables
    let local_port: u16 = args[1].parse().expect("Invalid local port");
    let gateway_ip: &String = &args[2];
    let remote_port: u16 = args[2].parse().expect("Invalid remote port");
    let username: &String = &args[4];
    let password: &String = &args[5];

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

    // Every time we send data to the local port,
    // spawn a thread with that data
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(client_stream) => {

                    // Create a channel to the remote server
                    let mut channel = 
                        session.channel_direct_tcpip(gateway_ip, remote_port, None);
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    });

    // Keep the main thread running
    loop {
        thread::park();
    }

    return Ok(());
}
