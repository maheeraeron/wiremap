use std::net::TcpStream;
use ssh2::Session;

fn foo() -> Result<(), Box<dyn std::error::Error>> {

    let gateway_ip = "dev.wiremap.io";
    let username = "root";
    let password = "Webma$ter1";
    
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

    Ok(());
}
