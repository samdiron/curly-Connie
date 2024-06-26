/*
will analyz the original file before writing it here
it's 730 lines cant do it in one time 
*/

use bytes::Bytes;
use rml_rtmp::chunk_io::Packet;
use rml_rtmp::sessions::StreamMetadata;
use rml_rtmp::sessions::{
    ServerSession, ServerSessionConfig, ServerSessionEvent, ServerSessionResult,
};
use rml_rtmp::time::RtmpTimestamp;
use slab::Slab;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;


enum ClientAction {
    Waiting,
    Publishing(String), // Publishing to a stream key
    Watching { stream_key: String, stream_id: u32 },
}

enum ReceivedDataType {
    Audio,
    Video,
    //Text,
}

struct Client {
    session: ServerSession,
    current_action: ClientAction,
    connection_id: usize,
    has_received_video_keyframe: bool,
}


impl Client {
    fn get_active_stream_id(&self) -> Option<u32> {
        match self.current_action {
            ClientAction::Waiting => None,
            ClientAction::Publishing(_) => None,
            ClientAction::Watching {
                stream_key: _,
                stream_id,
            } => Some(stream_id),
        }
    }
}

struct MediaChannel {
    publishing_client_id: Option<usize>,
    watching_client_ids: HashSet<usize>,
    metadata: Option<Rc<StreamMetadata>>,
    video_sequence_header: Option<Bytes>,
    audio_sequence_header: Option<Bytes>,
}

#[derive(Debug)]
pub enum ServerResult {
    DisconnectConnection {
        connection_id: usize,
    },
    OutboundPacket {
        target_connection_id: usize,
        packet: Packet,
    },
}

pub struct Server {
    clients: Slab<Client>,
    connection_to_client_map: HashMap<usize, usize>,
    channels: HashMap<String, MediaChannel>,
}
