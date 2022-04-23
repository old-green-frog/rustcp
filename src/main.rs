use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process;

fn main() {
    let server: Vec<_> = env::args().collect();
    let server = match server.len() {
        1 => {
            println!("Please, specify the server!");
            process::exit(1)
        }
        2 => &server[1],
        _ => {
            println!("Incorrect input!");
            process::exit(1)
        }
    };

    let mut stream = match TcpStream::connect(server) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect: {}", e);
            process::exit(0)
        }
    };
    println!("Connection established.");

    loop {
        let mut buffer = String::default();
        let mut stream_reader = BufReader::new(stream.try_clone().unwrap());

        std::thread::spawn(move || {
            match stream_reader.read_line(&mut buffer) {
                Ok(num) => {
                    if num > 0 {
                        if buffer != "" {
                            println!("Server message: '{}'", buffer.trim())
                        }
                    } else {
                        println!("Connection closed.");
                        process::exit(0)
                    }
                }
                Err(e) => {
                    println!("Something wrong: {}", e);
                    process::exit(0)
                }
            }
        });

        let mut input_buffer = String::default();
        match io::stdin().read_line(&mut input_buffer) {
            Ok(num) => {
                if num > 0 && input_buffer != "" {
                    stream.write(input_buffer.as_bytes()).unwrap();
                } else {
                    return
                }
            },
            Err(e) => {
                println!("Something wrong: {}", e);
                process::exit(0)
            }
        }
    }
}