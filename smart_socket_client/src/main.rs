use std::{
    io::{Read, Write},
    net::TcpStream,
};

use command::Command;

mod command;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        eprintln!("Usage: {} <host> <port>", args[0]);
        return;
    }

    let host = args[1].clone();
    let port = args[2].clone();
    let command = match &args[3] as &str {
        "get_info" => Command::GetDeviceInfo,
        "turn_on" if args.len() == 5 => {
            let device_id = args[4].parse().unwrap();
            Command::TurnOn(device_id)
        }
        "turn_off" if args.len() == 5 => {
            let device_id = args[4].parse().unwrap();
            Command::TurnOff(device_id)
        }
        "get_power" if args.len() == 5 => {
            let device_id = args[4].parse().unwrap();
            Command::GetPower(device_id)
        }
        "turn_on_all" => Command::TurnOnAll,
        "turn_off_all" => Command::TurnOffAll,
        _ => Command::Undefined,
    };

    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();
    println!("Connecting to server");

    let command_bytes = command.to_bytes();
    stream.write_all(&command_bytes).unwrap();

    println!("Sent command to server: {:?}", command);

    let mut buf = [0; 1024];

    let data = stream.read(&mut buf).unwrap();

    let response = std::str::from_utf8(&buf[0..data]).unwrap();

    println!("Received response: {:?}", response);

    // GetDeviceInfo,
    // TurnOff(u8),
    // TurnOn(u8),
    // GetPower(u8),
    // TurnOffAll,
    // TurnOnAll,
    // Undefined,
}
