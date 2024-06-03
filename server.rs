use std::io::*;
use std::net::*;


fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    // Read the Incoming data from a client and store it in the buffer
    stream.read(&mut buffer).expect("Failed to read from stream");
    // convert the data in the buffer 
    let data_received = String::from_utf8_lossy(&buffer[..]);
    // Print the data received from the client
    println!("Received: {}", data_received);

    // Write the data received back to the client
    let response = ("[SERVER]: msg received ").as_bytes();
    stream.write(response).expect("Failed to write to response");   
}
// establishing the server
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port");
    println!("Listening on: 127.0.0.1:8000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                
            }
        }
    }
}

