use std::{
    io,
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{self, AtomicBool, Ordering},
        Arc,
    },
    thread,
};

// ----------------------------------------------------------------------------

pub struct TCPListener<F1, F2, F3, F4>
where
    F1: FnMut() + Copy + Send + Sync + 'static,
    F2: FnMut() + Copy + Send + Sync + 'static,
    F3: FnMut(io::Error) + Copy + Send + Sync + 'static,
    F4: Fn(TcpStream) + 'static,
{
    addr: String,
    r: Arc<AtomicBool>,
    h: Option<thread::JoinHandle<()>>,

    // callback for events
    cb_error: Option<F3>,
    cb_opened: Option<F1>,
    cb_stopped: Option<F2>,
    cb_connected: Option<F4>,
}

impl<F1, F2, F3, F4> TCPListener<F1, F2, F3, F4>
where
    F1: FnMut() + Copy + Send + Sync + 'static,
    F2: FnMut() + Copy + Send + Sync + 'static,
    F3: FnMut(io::Error) + Copy + Send + Sync + 'static,
    F4: Fn(TcpStream) + 'static,
{
    pub fn new(addr: String) -> Self {
        Self {
            addr,
            r: Arc::new(AtomicBool::new(true)),
            h: None,

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
                let r = self.r.clone();

                let mut cb_error = self.cb_error.clone();
                //  let mut cb_opened = self.cb_opened.clone();
                let mut cb_stopped = self.cb_stopped.clone();
                let mut cb_connected = self.cb_connected;

                let h = std::thread::spawn(move || {
                    loop {
                        // listener
                        match listener.accept() {
                            // new connection
                            Ok((stream, _peer_addr)) => {
                                stream.set_nonblocking(true).unwrap();

                                if let Some(cb_connected) = &mut cb_connected {
                                    cb_connected(stream);
                                }
                            }

                            // error
                            Err(err) => {
                                eprintln!("accept failed: err = {}", err);

                                if let Some(cb_error) = &mut cb_error {
                                    cb_error(err);
                                }
                            }
                        }

                        // quit
                        if !r.load(atomic::Ordering::SeqCst) {
                            if let Some(cb_stopped) = &mut cb_stopped {
                                cb_stopped();
                            }

                            println!("fuckyou");

                            break;
                        }
                    }
                });

                self.h = Some(h);
            }

            Err(err) => {
                eprintln!("listen failed: err = {}, addr = {}", err, &self.addr);

                if let Some(cb_error) = &mut self.cb_error {
                    cb_error(err);
                }

                return false;
            }
        }

        if let Some(cb_opened) = &mut self.cb_opened {
            cb_opened();
        }

        true
    }

    pub fn stop(&mut self) {
        self.r.store(false, Ordering::SeqCst);

        if let Some(h) = self.h.take() {
            h.join().unwrap();
        }

        println!("tcp server stoped");
    }
}
