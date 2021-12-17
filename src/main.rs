use std::{
    cell::RefCell,
    io::{self, Result},
    sync::{Arc, Mutex},
};

mod looper;
mod net_mgr;
mod network;

use looper::Looper;
use net_mgr::NetMgr;
use network::{TCPListener, TCPSession};

// type Shared<T> = Arc<Mutex<RefCell<T>>>;

fn main() -> Result<()> {
    println!("server start ...");

    let nmgr = Arc::new(Mutex::new(RefCell::new(NetMgr::new())));

    let nmgr2 = nmgr.clone();

    // thread listener
    let mut listener = TCPListener::new(String::from("0.0.0.0:8080"))
        .set_event_opened(|| {
            println!("opened");
        })
        .set_event_stopped(|| {
            println!("stoped");
        })
        .set_event_error(|err| {
            println!("err {}", err);
        })
        .set_event_connected(move |stream| {
            println!("on connected: {:?}", stream);

            let session = TCPSession::new(stream);

            let p = nmgr2.lock().unwrap();
            let mut n = p.borrow_mut();
            n.new_connection(session);
        });

    listener.start();

    // main thread loop
    let mut looper = Looper::new();
    {
        let nmgr = nmgr.clone();

        looper.run(move || {
            let p = nmgr.lock().unwrap();
            let mut n = p.borrow_mut();
            n.update();
        });
    }

    // read from stdin
    let mut input = String::new();

    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                println!("input str = {}", input);

                if input.eq("quit") {
                    break;
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }

    println!("server will be close");

    listener.stop();
    looper.stop();

    {
        let p = nmgr.lock().unwrap();
        let mut n = p.borrow_mut();
        n.stop();
    }

    println!("server closed");
    Ok(())
}
