// this file is not working for now 

use tokio::io::*;
use rml_rtmp::{RtmpServer, Connect, AMF0Decode, AMF0Encode};
use std::{fs::*, process::Output};
use tokio::sync::*;
use ffmpeg_sys::*;

async fn process_data(data: &[u8]) {

    //process Received data with ffmpeg 
    let mut codec_context: AVCodecContext = unsafe { avcodec_alloc_context3(None) };
    let mut encoder: *mut AVCodecContext = codec_context.as_mut_ptr();
     
    let h264_encoder = unsafe {
        ffmpeg_sys::avcodec_find_encoder_by_name("libx264")
    };
    if h264_encoder.is_null() {
        panic!("Failed to find H264 encoder");
    }

     unsafe {
         (*encoder).bits_per_raw_sample = (*h264_encoder).bits_per_raw_sample;
         (*encoder).codec_id = (*h264_encoder).id;
         (*encoder).codec_type = (*h264_encoder).codec_type;
         (*encoder).pix_fmt = (*h264_encoder).pix_fmt;
         (*encoder).time_base = (*h264_encoder).time_base;

     }
     let ret = ffmpeg_sys::avcodec_open2(encoder, h264_encoder, None);
     if ret < 0 {
         panic!("Failed to open H264 encoder");

     }
    
    let mut input_packet:AVPacket::default();
    unsafe {
        ffmpeg_sys::av_new_video_frame(&mut input_packet);
    }
    input_packet.data = data.as_ptr() as *mut u8;
    input_packet.size = data.len();
    let mut output_packet:AVPacket::default();
    unsafe {
        ffmpeg_sys::av_new_video_frame(&mut output_packet);
        let ret = ffmpeg_sys::avcodec_encode_video2(encoder, &mut output_packet, &mut input_packet, None);
        if ret < 0 {
            panic!("Failed to encode video frame");
        }
    }
    let mut encoded_data = vec![0; output_packet];
    unsafe {
        std::ptr::copy(output_packet.data, encoded_data.as_mut_ptr(), output_packet.size)
    }

    //cleanup after the encoding
    unsafe {
        ffmpeg_sys::av_free_packet(&mut input_packet);
        ffmpeg_sys::av_free_packet(&mut output_packet);
        ffmpeg_sys::avcodec_free_context(&mut codec_context);
    }
    Ok(encoded_data);
    println!("hip hip horay");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Port to listen on (standard RTMP port is 1935)
    let port = 1935;
    //initialize ffmpeg 
    ffmpeg_sys::init();
    
    // Create a new RTMP server instance
    let mut server = RtmpServer::listen(format!("127.0.0.1:{}", port))?;

    println!("RTMP server listening on port {}", port);

    // Accept incoming connections
    loop {
        let (mut stream) = server.accept().await?;

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

                    let (tx, rx) = mpsc::channel::<Vec<u8>>(); // Create a channel for data
                    let media_file = "test.mp4";
                    let file = tokio::fs::File::open(media_file).await?;
                    let mut reader = BufReader::new(file);
                    let mut buffer = Vec::new();
                    

                    // Open the media file
                    let media_file = "test.mp4";
                    let file = tokio::fs::File::open(media_file).await?;

                    // Create a buffered reader for efficient reading
                    let mut reader = BufReader::new(file);

                    // Buffer to store read data
                    let mut buffer = Vec::new();

                    // Spawn a separate task to asynchronously read the MP4 file
                    tokio::spawn(async move {
                        loop {
                            // Read data into the buffer
                            let n = reader.read_buf(&mut buffer).await?;

                            // Check for  
                        }
                        // Send the buffer to the channel
                        tx.send(buffer.clone()).await?;
                        // Clear the buffer for the next read
                        buffer.clear();
                    });
                    
                   
                    loop {
                        let outdata = rx.recv().await.unwrap();
                        let packet = process_data(outdata).await?;
                        stream.send_packet(Packet::new(packet.as_slice())).await?;
                    };
                    
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
