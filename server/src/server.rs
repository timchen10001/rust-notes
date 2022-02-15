use std::{io::Read, net::TcpListener};

pub struct Server {
    address: String,
}

fn arr(a: &[u8]) {}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Server listening on: {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut tcpStream, _socketAddr)) => {
                    let mut buffer: [u8; 1024] = [0; 1024];

                    match tcpStream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieve a request: {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(ex) => {
                            println!("Failed to read from connection: {}", ex);
                        }
                    };
                }
                Err(ex) => println!("Failed to establish a connection: {}", ex),
            }
        }
    }
}
