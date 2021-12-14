use std::{io, net::TcpStream};

pub struct TCPConnector {
    stream: Option<TcpStream>,
}

impl TCPConnector {
    pub fn new() -> Self {
        Self { stream: None }
    }

    // 开单独线程来连接，connect不支持异步
    pub fn connect(&mut self, addr: String) -> bool {
        match TcpStream::connect(addr) {
            Ok(stream) => {
                self.stream = Some(stream);
                true
            }

            Err(err) => {
                eprintln!("connect failed: err = {}", err);
                false
            }
        }
    }
}
