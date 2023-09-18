use core::str::FromStr;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

enum Functional {
    Current,
}
impl FromStr for Functional {
    type Err = ();
    fn from_str(input: &str) -> Result<Functional, Self::Err> {
        match input {
            "current" => Ok(Functional::Current),
            _ => Err(()),
        }
    }
}

pub trait Thermostat {
    fn current_temperature(&self) -> i32;
}
pub struct Th {}
impl Thermostat for Th {
    fn current_temperature(&self) -> i32 {
        // ... Здесь реаллизуем нужный функционал
        32
    }
}

fn handle_client(mut stream: TcpStream) {
    let thermostat = Th {};

    let mut buffer = String::new();
    let _ = stream.read_to_string(&mut buffer);

    let functional = Functional::from_str(buffer.to_lowercase().trim());

    match functional {
        Ok(Functional::Current) => {
            println!("Current temperature: {}", thermostat.current_temperature())
        }
        Err(()) => println!("Use commands:\n - current\n"),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
