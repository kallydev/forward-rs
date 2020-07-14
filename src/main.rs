use std::{env, io};
use std::net;
use std::thread;

struct Server {
    client: String,
    server: String,
}

trait IServer {
    fn new(client: String, server: String) -> Server;
    fn run(&self);
}

impl IServer for Server {
    fn new(client: String, server: String) -> Server {
        Server { client, server }
    }

    fn run(&self) {
        let (client, server) = (self.client.clone(), self.server.clone());
        let listener = net::TcpListener::bind(client).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(src) => {
                    let server = server.clone();
                    thread::spawn(|| {
                        match net::TcpStream::connect(server) {
                            Ok(dst) => {
                                forward(src, dst);
                            }
                            Err(error) => {
                                eprintln!("{}", error);
                            }
                        }
                    });
                }
                Err(error) => {
                    eprintln!("{}", error);
                }
            }
        }
    }
}

fn forward(src: net::TcpStream, dst: net::TcpStream) {
    let (mut src_read, mut src_write) = (src.try_clone().unwrap(), src.try_clone().unwrap());
    let (mut dst_read, mut dst_write) = (dst.try_clone().unwrap(), dst.try_clone().unwrap());
    let threads = vec![
        thread::spawn(move || match io::copy(&mut src_read, &mut dst_write) {
            _ => {}
        }),
        thread::spawn(move || match io::copy(&mut dst_read, &mut src_write) {
            _ => {}
        }),
    ];
    for t in threads {
        t.join().unwrap();
        match src.shutdown(net::Shutdown::Both) {
            Err(error) => {
                eprintln!("{}", error);
            }
            _ => {}
        }
        match dst.shutdown(net::Shutdown::Both) {
            Err(error) => {
                eprintln!("{}", error);
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("forward <client_addr> <server_addr>");
        return;
    }
    Server::new(args[1].to_string(), args[2].to_string()).run();
}
