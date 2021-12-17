use std::net::{Shutdown, TcpStream};

pub struct TCPSession {
    id: i32,
    stream: Option<TcpStream>,
    buff_reader: [u8; 4096],
}

impl TCPSession {
    pub fn new(stream: TcpStream) -> TCPSession {
        Self {
            id: 0,
            stream: Some(stream),
            buff_reader: [0; 4096],
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    // read & parse & dispatch
    pub fn update(&mut self) {
        self.read();
        self.write();
    }

    pub fn send_bytes(&mut self) {}
    pub fn send_packet(&mut self) {}
    pub fn send_protobuf(&mut self) {}

    pub fn close(&mut self) {
        if let Some(stream) = &mut self.stream {
            stream.shutdown(Shutdown::Both);
        }
    }
}

// private
impl TCPSession {
    // read
    fn read(&mut self) {
        if self.stream.is_none() {
            return;
        }

        if let Some(stream) = &mut self.stream {
            stream.read_exact()
        }
    }

    fn write(&mut self) {}
}
