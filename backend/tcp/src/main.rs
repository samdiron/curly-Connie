use std::fs;
use std::io;
use std::io::*;
use std::net::*;


fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("client connected");
    let filename = "path/to/a/file";

    let mut file = fs::File::open(filename)?;
    let file_meta = fs::metadata(filename)?;
  
    // Send file size as u64
    let file_size = file_meta.len();
    println!("size: {}",file_size);
    stream.write_all(&file_size.to_be_bytes())?;
    println!("file size sent");


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
    
}


// establishing the server
fn main(){
    let listener = TcpListener::bind("IP:8000").expect("Failed to bind to port");
    println!("Listening on: IP:8000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {  
                    println!("handle client");  
                    let _ = handle_client(stream);
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
tested with the tcp-client from client backend 
took 2 second to the file to be writen in the client i think that's good 
note the filesize is 6 MGB and it could be faster if we removed the println functions
this need to be modified to also send more info about the file like the extention, name, image and embeded
info we will work on that when making the actual Connie
*/
