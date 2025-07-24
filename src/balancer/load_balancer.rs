use core::{option::Option::None, result::Result::Err};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::strategies::strategy::Strategy;

pub struct LoadBalancer {
    strategy: Box<dyn Strategy>,
}

// Probably serve and handle must not be a part of LoadBalancer
impl LoadBalancer {
    pub fn new(strategy: impl Strategy + 'static) -> Self {
        LoadBalancer {
            strategy: Box::new(strategy),
        }
    }

    pub fn serve(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:8000")?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle(stream),
                Err(e) => eprintln!("Connection failed {}", e),
            }
        }
        Ok(())
    }

    pub fn handle(&self, mut stream: TcpStream) {
        match self.strategy.get_next_server() {
            Some(server) => {
                println!("{}", server.name);
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let _ = stream.write(format!("Redirecting to {}", server.name).as_bytes());
                    }
                    Err(_) => println!("!"),
                }
            },
            None => println!("none"),
        }
    }
}
