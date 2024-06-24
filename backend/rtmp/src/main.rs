/*
! use the rml_rtmp lib , example of it in examples/threaded_rtmp_server/src in github rust-media-libs
*/
use std::net::{TcpListener, };//TcpStream};
//use rml_rtmp::*;
//use std::thread;





fn main(){
    let listener: TcpListener = TcpListener::bind("192.168.7.13:8080").expect("fail");
    println!("ok");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {  
                    println!("handle client");  
                    //let _ = handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                
            }
        }
    }
    
}