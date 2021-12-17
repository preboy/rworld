use std::{
    io::{self, ErrorKind},
    net::{TcpListener, TcpStream},
};

// ----------------------------------------------------------------------------

pub struct TCPListener<F1, F2, F3, F4>
where
    F1: Fn() + 'static,
    F2: Fn() + 'static,
    F3: Fn(io::Error) + 'static,
    F4: Fn(TcpStream) + 'static,
{
    addr: String,
    listener: Option<TcpListener>,

    // callback for events
    cb_error: Option<F3>,
    cb_opened: Option<F1>,
    cb_stopped: Option<F2>,
    cb_connected: Option<F4>,
}

impl<F1, F2, F3, F4> TCPListener<F1, F2, F3, F4>
where
    F1: Fn() + 'static,
    F2: Fn() + 'static,
    F3: Fn(io::Error) + 'static,
    F4: Fn(TcpStream) + 'static,
{
    pub fn new(addr: String) -> Self {
        Self {
            addr,
            listener: None,

            // callback for events
            cb_error: None,
            cb_opened: None,
            cb_stopped: None,
            cb_connected: None,
        }
    }

    pub fn set_event_error(mut self, cb: F3) -> Self {
        self.cb_error = Some(cb);
        self
    }

    pub fn set_event_opened(mut self, cb: F1) -> Self {
        self.cb_opened = Some(cb);
        self
    }

    pub fn set_event_stopped(mut self, cb: F2) -> Self {
        self.cb_stopped = Some(cb);
        self
    }

    pub fn set_event_connected(mut self, cb: F4) -> Self {
        self.cb_connected = Some(cb);
        self
    }

    pub fn start(&mut self) -> bool {
        match TcpListener::bind(&self.addr) {
            Ok(listener) => {
                listener.set_nonblocking(true).unwrap();
                self.listener = Some(listener);

                if let Some(cb_opened) = &mut self.cb_opened {
                    cb_opened();
                };

                true
            }

            Err(err) => {
                eprintln!("listen failed: err = {}, addr = {}", err, &self.addr);
                if let Some(cb_error) = &mut self.cb_error {
                    cb_error(err);
                }

                false
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(listener) = &mut self.listener {
            // listener
        };

        if let Some(cb_stopped) = &mut self.cb_stopped {
            cb_stopped();
        };

        println!("tcp server stoped");
    }

    pub fn update(&mut self) {
        if let Some(listener) = &self.listener {
            match listener.accept() {
                // new connection
                Ok((stream, _peer_addr)) => {
                    stream.set_nonblocking(true).unwrap();

                    if let Some(cb_connected) = &self.cb_connected {
                        cb_connected(stream);
                    }
                }

                // error
                Err(err) => {
                    if err.kind() != ErrorKind::WouldBlock {
                        eprintln!("accept failed: err = {}", err);

                        if let Some(cb_error) = &mut self.cb_error {
                            cb_error(err);
                        }
                    }
                }
            }
        }
    }
}
