use std::thread;
use std::time::Duration;

pub struct Fuck<T>
where
    T: Fn(i32) + Send + 'static,
{
    f: Option<T>,
    h: Option<thread::JoinHandle<()>>,
}

impl<T> Fuck<T>
where
    T: Fn(i32) + Send + 'static,
{
    pub fn new(addr: String) -> Self {
        Self { f: None, h: None }
    }

    pub fn set(self, f: T) -> Self {
        self.f = Some(f);
        self
    }

    pub fn start(&mut self) {
        let mut f = self.f;

        let h = thread::spawn(move || loop {
            println!("in thread");

            if let Some(f) = f {
                f(1);
            }

            std::thread::sleep(Duration::from_millis(100));
        });

        self.h = Some(h);
    }

    pub fn stop(&mut self) {
        if let Some(h) = self.h {
            h.join().unwrap();
        }
    }
}
