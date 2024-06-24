
/*
note this code is not mine it's from (rust-media-libs/examples/threaded_rtmp_server/src
/connection.rs) but it's for studing how the rml_rtmp works.
this is for handling the RTMP handshake it's not complete will not commit the other files it has my info so will not do that now 

*/



use rml_rtmp::handshake::{Handshake, HandshakeProcessResult, PeerType};
use std::collections::VecDeque;
use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

const BUFFER_SIZE: usize = 4096;

pub enum ReadResult {
    HandshakingInProgress,
    NoBytesReceived,
    BytesReceived {
        buffer: [u8; BUFFER_SIZE],
        byte_count: usize,
    },
}

#[derive(Debug)]
pub enum ConnectionError {
    IoError(io::Error),
    SocketClosed,
}

impl From<io::Error> for ConnectionError {
    fn from(error: io::Error) -> Self {
        ConnectionError::IoError(error)
    }
}

pub struct Connection {
    pub connection_id: usize,
    writer: Sender<Vec<u8>>,
    reader: Receiver<ReadResult>,
    handshake: Handshake,
    handshake_completed: bool,
}

impl Connection {

    
    pub fn new(connection_id: usize, socket: TcpStream) -> Connection {
        let (byte_sender, byte_receiver) = channel();
        let (result_sender, result_receiver) = channel();

        start_byte_writer(byte_receiver, &socket);
        start_result_reader(result_sender, &socket);

        Connection {
            connection_id,
            writer: byte_sender,
            reader: result_receiver,
            handshake: Handshake::new(PeerType::Server),
            handshake_completed: false,
        }
    }

    pub fn write(&self, bytes: Vec<u8>) {
        self.writer.send(bytes).unwrap();
    }

    pub fn read(&mut self) -> Result<ReadResult, ConnectionError> {
        match self.reader.try_recv() {
            Err(TryRecvError::Empty) => Ok(ReadResult::NoBytesReceived),
            Err(TryRecvError::Disconnected) => Err(ConnectionError::SocketClosed),
            Ok(result) => match self.handshake_completed {
                true => Ok(result),
                false => match result {
                    ReadResult::HandshakingInProgress => unreachable!(),
                    ReadResult::NoBytesReceived => Ok(result),
                    ReadResult::BytesReceived { buffer, byte_count } => {
                        self.handle_handshake_bytes(&buffer[..byte_count])
                    }
                },
            },
        }
    }



}