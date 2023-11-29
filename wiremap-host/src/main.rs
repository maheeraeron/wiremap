// Imports
use ssh2::{Channel, Session};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, env};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // For verbose logging
    env::set_var("RUST_BACKTRACE", "full");

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

        // Send the incoming request over ssh on to the remote localhost and port
        let mut channel: Channel = session
            .channel_direct_tcpip("localhost", remote_port, None)
            .unwrap();
        channel.write(&request).unwrap();
    }
    return Ok(());
}

// fn handle_client(mut local_stream: TcpStream, mut remote_channel: Channel) {
//     // Buffer for local stream
//     let mut local_buf = [0; 4096];

//     loop {
//         // Read from the local client
//         let local_read_size = local_stream.read(&mut local_buf).unwrap();
//         if local_read_size == 0 {
//             break; // End of file (EOF) from the local client
//         }

//         // Write to the remote channel
//         println!("Writing data to 10004 on remote...");
//         let remote_write_size = remote_channel.write(&local_buf[..local_read_size]).unwrap();
//         if remote_write_size == 0 {
//             break; // Error writing to the remote channel
//         }
//     }
// }
