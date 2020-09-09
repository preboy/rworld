use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{
    atomic::{self, AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time;

// ----------------------------------------------------------------------------

pub trait SessionHolder {
    fn set_session(&self, s: Option<&Session>);
}

// ----------------------------------------------------------------------------

pub struct Session {
    sock: Option<TcpStream>,
    sendq: Vec<u8>,
    //  holder: Option<Box<dyn SessionHolder>>,
}

impl Session {
    pub fn new(ts: TcpStream) -> Session {
        Session {
            sock: Some(ts),
            sendq: vec![],
            // holder: None,
        }
    }

    fn update(&mut self) -> bool {
        if self.read() {
            return true;
        }

        if self.write() {
            return true;
        }

        false
    }

    fn read(&mut self) -> bool {
        if let Some(ref mut sock) = self.sock {
            let mut buff = [0u8; 1024];

            match sock.read(&mut buff) {
                Ok(n) if n == 0 => {
                    println!("soclet closed");
                    return true;
                }

                Ok(n) => {
                    println!("{} bytes read, data = {:?}", n, &buff[..n]);
                    // self.post(&buf[..n]);
                }

                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                }

                Err(e) => {
                    eprintln!("encountered IO error: {}", e);
                    return true;
                }
            }
        }

        false
    }

    fn write(&mut self) -> bool {
        if let Some(ref mut sock) = self.sock {
            if self.sendq.is_empty() {
                return false;
            }

            if !self.sendq.is_empty() {
                let cnt = sock.write(&self.sendq[..]).unwrap();
                self.sendq.clear();
                println!("{} bytes writen", cnt);
            }
        }

        false
    }

    pub fn post(&mut self, data: &[u8]) {
        self.sendq.extend_from_slice(data);
    }
}

// ----------------------------------------------------------------------------

pub struct SessionMgr {
    ss: HashMap<u32, Session>,
}

impl SessionMgr {
    pub fn new() -> SessionMgr {
        SessionMgr { ss: HashMap::new() }
    }

    pub fn add(&mut self, id: u32, sess: Session) {
        self.ss.insert(id, sess);
    }

    pub fn get(&mut self, id: u32) -> Option<&Session> {
        self.ss.get(&id)
    }

    pub fn del(&mut self, id: u32) {
        self.ss.remove(&id);
    }

    pub fn update(&mut self) {
        let mut r = None;

        for (k, v) in self.ss.iter_mut() {
            let err = v.update();
            if err {
                r = Some(*k);
                break;
            }
        }

        if let Some(k) = r {
            self.ss.remove_entry(&k);
        }
    }
}

// ----------------------------------------------------------------------------

pub struct TCPServer {
    addr: String,
    r: Arc<AtomicBool>,
    h: Option<thread::JoinHandle<()>>,
    sess_mgr: Arc<Mutex<SessionMgr>>,
}

impl TCPServer {
    pub fn new(addr: String) -> TCPServer {
        TCPServer {
            addr,
            r: Arc::new(AtomicBool::new(false)),
            h: None,
            sess_mgr: Arc::new(Mutex::new(SessionMgr::new())),
        }
    }

    pub fn start(&mut self) -> bool {
        match TcpListener::bind(&self.addr) {
            Ok(listener) => {
                let r = self.r.clone();
                let sess_mgr = self.sess_mgr.clone();

                let h = std::thread::spawn(move || {
                    listener.set_nonblocking(true).unwrap();
                    let mut idx: u32 = 0;

                    loop {
                        // listener
                        match listener.accept() {
                            Ok((stream, _peer_addr)) => {
                                stream.set_nonblocking(true).unwrap();
                                idx = idx + 1;

                                if let Ok(ref mut mgr) = sess_mgr.lock() {
                                    mgr.add(idx, Session::new(stream));
                                }
                            }

                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // println!("encountered IO error: {}", e);
                            }

                            Err(e) => {
                                panic!("OTHER IO error: {}", e);
                            }
                        }

                        // session manager
                        {
                            let mut lock = sess_mgr.try_lock();
                            if let Ok(ref mut mgr) = lock {
                                mgr.update();
                            }
                        }

                        // breaker
                        if !r.load(atomic::Ordering::SeqCst) {
                            break;
                        }

                        thread::sleep(time::Duration::from_millis(10));
                    }
                });

                self.h = Some(h);
            }

            Err(err) => {
                eprintln!("lisnten failed: err = {}, addr = {}", err, &self.addr);
                return false;
            }
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
