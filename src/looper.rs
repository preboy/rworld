use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub struct Looper {
    h: Option<thread::JoinHandle<()>>,
    r: Arc<AtomicBool>,
}

impl Looper {
    pub fn new() -> Self {
        Self {
            h: None,
            r: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run<T>(&mut self, f: T)
    where
        T: Fn() + 'static + Send,
    {
        self.r.store(true, Ordering::SeqCst);
        let r = self.r.clone();

        let h = thread::spawn(move || loop {
            f();

            if !r.load(Ordering::SeqCst) {
                break;
            };

            thread::sleep(Duration::from_millis(1));
        });

        self.h = Some(h);
    }

    pub fn stop(&mut self) {
        self.r.store(false, Ordering::SeqCst);

        if let Some(h) = self.h.take() {
            h.join().unwrap();
        }

        println!("looper stopped");
    }
}
