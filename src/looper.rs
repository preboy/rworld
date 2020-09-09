use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};

use std::thread;
use std::time::Duration;

pub struct Looper {
    h: Option<thread::JoinHandle<()>>,
    tx: Option<mpsc::Sender<i32>>,
    r: Arc<AtomicBool>,
}

impl Looper {
    pub fn new() -> Looper {
        Looper {
            h: None,
            tx: None,
            r: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.tx = Some(tx);

        self.r.store(true, Ordering::SeqCst);
        let r = self.r.clone();

        let h = thread::spawn(move || loop {
            if let Ok(v) = rx.try_recv() {
                println!("recv values: {}", v);
            } else {
                thread::sleep(Duration::from_micros(1));
            };

            if !r.load(Ordering::SeqCst) {
                break;
            };
        });

        self.h = Some(h);
    }

    pub fn post(&self, v: i32) {
        if let Some(tx) = &self.tx {
            tx.send(v).unwrap();
        }
    }

    pub fn stop(&mut self) {
        self.r.store(false, Ordering::SeqCst);

        if let Some(h) = self.h.take() {
            h.join().unwrap();
        }

        println!("looper stoped");
    }
}
