use std::io;

mod looper;
mod network;

fn main() -> io::Result<()> {
    println!("server start ...");

    // thread listener
    let mut server = network::TCPServer::new(String::from("0.0.0.0:8080"));
    server.start();

    // thread loop
    let mut looper = looper::Looper::new();
    looper.run();

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

    server.stop();
    looper.stop();

    println!("server closed");
    Ok(())
}
