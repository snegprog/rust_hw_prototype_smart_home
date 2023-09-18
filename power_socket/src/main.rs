use core::str::FromStr;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

enum Functional {
    Description,
    CurrentPower,
    On,
    Off,
}
impl FromStr for Functional {
    type Err = ();
    fn from_str(input: &str) -> Result<Functional, Self::Err> {
        match input {
            "description" => Ok(Functional::Description),
            "current-power" => Ok(Functional::CurrentPower),
            "on" => Ok(Functional::On),
            "off" => Ok(Functional::Off),
            _ => Err(()),
        }
    }
}

pub trait PowerSocket {
    fn description(&self) -> &str;
    fn current_power(&self) -> i32;
    fn on(&self) -> bool;
    fn off(&self) -> bool;
}
pub struct PS {}
impl PowerSocket for PS {
    fn description(&self) -> &str {
        // ... Здесь реаллизуем нужный функционал
        "Description"
    }

    fn current_power(&self) -> i32 {
        // ... Здесь реаллизуем нужный функционал
        32
    }

    fn on(&self) -> bool {
        // ... Здесь реаллизуем нужный функционал
        true
    }
    fn off(&self) -> bool {
        // ... Здесь реаллизуем нужный функционал
        true
    }
}

fn handle_client(mut stream: TcpStream) {
    let power_socket = PS {};

    let mut buffer = String::new();
    let _ = stream.read_to_string(&mut buffer);

    let functional = Functional::from_str(buffer.to_lowercase().trim());

    match functional {
        Ok(Functional::Description) => {
            println!("Description: {}", power_socket.description())
        }
        Ok(Functional::CurrentPower) => {
            println!("Power: {}", power_socket.current_power())
        }
        Ok(Functional::On) => {
            println!("Power: on")
        }
        Ok(Functional::Off) => {
            println!("Power: off")
        }
        Err(()) => println!("Use commands:\n - description\n - current-power\n - on\n - off\n"),
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
