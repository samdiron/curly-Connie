// ps not complete yet 
// a server for streaming media 
// imports
use std::io::{Error, ErrorKind};
use rml_rtmp::{RtmpServer, Connect, AMF0Decode, AMF0Encode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Port to listen on (standard RTMP port is 1935)
    let port = 1935;

    // Create a new RTMP server instance
    let mut server = RtmpServer::listen(format!("0.0.0.0:{}", port))?;

    println!("RTMP server listening on port {}", port);

    // Accept incoming connections
    loop {
        let (mut stream, _) = server.accept().await?;

        // Handle connection (replace with actual logic)
        match stream.handshake().await {
            Ok(Connect {..}) => println!("Client connected"),
            Err(err) => println!("Handshake error: {}", err),
        }

        // Process incoming messages (replace with actual logic)
        loop {
            let msg = stream.recv_message().await?;
            match msg {
                Some(message) => {
                    // Decode message payload (if needed)
                    let data = AMF0Decode::decode(&message.payload)?;
                    println!("Received message: {:?}", data);

                    // Send a dummy response (replace with actual logic)
                    let response = AMF0Encode::encode(&"Hello from server!".to_string())?;
                    stream.send_message(message.chunk_stream_id, &response).await?;
                },
                None => {
                    println!("Client disconnected");
                    break;
                }
            }
        }
    }

    Ok(())
}
