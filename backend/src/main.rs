use std::fs;
//imports
use std::io;
use std::io::*;
//use std::io::Read;
//use std::io::Write;
use std::net::*;

// today i was just doing research about networking and i think i learned something so tomorrow i will finish the tcp server and client and improve all the tcp functions and it's the easiest part of this project but i will do it and finish it no slaking 
//TODO add more info like image and more
/*struct Item {
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
*/

fn send_mp3_file(filename: &str, server_addr: &str) -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect(server_addr)?;
  
    // Open the MP3 file for reading
    
  
    // Shutdown the sending side of the connection
    stream.shutdown(Shutdown::Write)?;
  
    println!("Sent MP3 file: {}", filename);
    Ok(())
}
  

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let filename = "Door.mp3";

    let mut file = fs::File::open(filename)?;
    let mut file_meta = fs::metadata(filename)?;
  
    // Send file size as u64
    let file_size = file_meta.len();
    stream.write_all(&file_size.to_be_bytes())?;
  
    // Read file in chunks and send
    let mut buffer = [0; 4096];
    loop {
      let bytes_read = file.read(&mut buffer)?;
      if bytes_read == 0 {
        break;
      }
      stream.write_all(&buffer[..bytes_read])?;
    }
    // Shutdown the sending side of the connection
    stream.shutdown(Shutdown::Write)?;

    println!("Sent MP3 file: {}", filename);
    Ok(())
    
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
                    let cc = handle_client(stream);
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