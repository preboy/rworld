use std::{cell::RefCell, io::Result, rc::Rc, sync::Mutex, thread, time::Duration};

mod looper;
mod net_mgr;
mod network;

// use looper::Looper;
use net_mgr::NetMgr;
use network::{TCPListener, TCPSession};

fn main() -> Result<()> {
    println!("server start ...");

    let nmgr = Rc::new(RefCell::new(NetMgr::new()));

    let nmgr2 = nmgr.clone();

    let mut listener = TCPListener::new(String::from("0.0.0.0:8080"))
        .set_event_error(|err| {
            println!("listenr error: {}", err);
        })
        .set_event_opened(|| {
            println!("listenr opened");
        })
        .set_event_stopped(|| {
            println!("listenr stopped");
        })
        .set_event_connected(move |stream| {
            println!("listenr be connected: {:?}", stream);

            let mut nm = nmgr2.borrow_mut();
            nm.new_connection(TCPSession::new(stream));
        });

    listener.start();

    loop {
        listener.update();

        let mut nm = nmgr.borrow_mut();
        nm.update();

        thread::sleep(Duration::from_millis(1));
    }

    let mut nm = nmgr.borrow_mut();
    nm.stop();

    println!("server closed");
    Ok(())
}
