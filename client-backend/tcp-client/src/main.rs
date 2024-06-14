use std::fs;
use std::io;
use std::io::*;
use std::net::*;


fn main() -> io::Result<()> {
  let filename = "client.mp3";
  let mut stream = TcpStream::connect("127.0.0.1:8000").expect("not connecting");
  println!("connected");
  let mut file = fs::File::create(filename)?;

  // Receive file size as u64
  let mut file_size_buffer = [0; 8];
  stream.read(&mut file_size_buffer).expect("file size");
  let file_size = u64::from_be_bytes(file_size_buffer);
  println!("file size recv{}", file_size);
  
  // Receive and write file data in chunks
  let mut buffer = [0; 4096];
  let mut total_bytes_received = 0;
  while total_bytes_received < file_size {
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
      return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of stream"));
    }
    file.write_all(&buffer[..bytes_read])?;
    total_bytes_received += bytes_read as u64;
    println!("file writen");
  }
  
  println!("Received file: {}", filename);
  Ok(())
}
