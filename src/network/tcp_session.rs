use std::net::TcpStream;

pub struct TCPSession {
    id: i32,
    stream: Option<TcpStream>,
}

impl TCPSession {
    pub fn new(stream: TcpStream) -> TCPSession {
        Self {
            id: 0,
            stream: Some(stream),
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    // read & parse & dispatch
    pub fn update(&mut self) {}

    pub fn send_bytes(&mut self) {}
    pub fn send_packet(&mut self) {}
    pub fn send_protobuf(&mut self) {}
}

// private
impl TCPSession {
    fn read(&mut self) {}
}
