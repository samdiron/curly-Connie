use std::io::*;
use std::net::*;


fn main() {
    // Establishing connection
    let mut client:TcpStream = TcpStream::connect("127.0.0.1:8000").expect("Failed to connect");
    let mut buffer = [0; 1024];
   
    // Write to server
    let greeting = ("[CLIENT]: hi").as_bytes();
    client.write(greeting).expect("Failed to write to response");
    // Read the Incoming data from a server and store it in the buffer
    client.read(&mut buffer).expect("Failed to read from stream");
   
    // convert the data in the buffer
    let data_received = String::from_utf8_lossy(&buffer[..]);
       
    // Print the data received from the server
    println!("Received: {}", data_received);
}
