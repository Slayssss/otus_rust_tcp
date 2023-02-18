use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::command::Command;

mod command;

fn handle_client(mut stream: TcpStream) {
    println!("Received client: {}", stream.peer_addr().unwrap());

    loop {
        let mut buf = [0; 1024];

        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    println!("Client disconnected");
                    break;
                }

                match Command::from_bytes(&buf[..n]) {
                    Some(command) => {
                        println!("Received command: {:?}", command);

                        let response = match command {
                            Command::GetDeviceInfo => format!("Device information"),
                            Command::TurnOff(id) => {
                                format!("Smart Socket with id: {} is turned off", id)
                            }
                            Command::TurnOn(id) => {
                                format!("Smart Socket with id: {} is turned on", id)
                            }
                            Command::GetPower(id) => {
                                format!("Smart Socket with id: {} have power: 50", id)
                            }
                            Command::TurnOffAll => format!("All devices are turned off"),
                            Command::TurnOnAll => format!("All devices are turned on"),
                            Command::Undefined => format!("Unknown command"),
                        };

                        let response_bytes = response.as_bytes();

                        stream.write_all(response_bytes).unwrap();
                        println!("Sent response: {:?}", response);
                    }
                    None => {
                        println!("Received unknown command");
                    }
                }
            }

            Err(e) => {
                println!("Error reading socket {}", e);
                break;
            }
        }
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Server listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }

            Err(e) => {
                println!("Error reading stream: {}", e)
            }
        }
    }
}
