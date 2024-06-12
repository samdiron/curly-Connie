use std::fs;
//imports
use std::io::*;
use std::net::*;
use std::fs::*;

//TODO add more info like image and more
struct Item {
    name: String,
    path: String,
}



fn writer(mut stream:&TcpStream,&item :Item){
    let item_path = item.path.into_bytes();
    let item_name = item.name.into_bytes();
    stream.write(&item_name).expect("error name write");
    stream.write(&item_path).expect("path");
    stream.flush().expect("flush");

}


fn handle_client(mut stream: TcpStream){
    let mut buff = [0; 1080];
    let song = Item{name: "bag of bones".to_string(), path: "/path/to/a/song".to_string()};
    let path = song.path;
    /*let mut buffer = [0; 1024];
    // Read the Incoming data from a client and store it in the buffer
    stream.read(&mut buffer).expect("Failed to read from stream");
    // convert the data in the buffer 
    let data_received = String::from_utf8_lossy(&buffer[..]);
    // Print the data received from the client
    println!("Received: {}", data_received);

    // Write the data received back to the client
    let response = ("[SERVER]: msg received ").as_bytes();
    stream.write(response).expect("Failed to write to response");
    */   
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
/*
notes 
TODO make a function to read all files that could be sent to the client

*/