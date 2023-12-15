use std::{
    thread,
    net::{TcpStream},
    io::{BufReader, Write, BufRead, self},
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut stream_clone = stream.try_clone().unwrap();

    thread::spawn(move || {
        let mut reader = BufReader::new(&mut stream_clone);
        loop {
            let mut buffer = String::new();
            if let Ok(read_bytes) = reader.read_line(&mut buffer) {
                if read_bytes > 0 {
                    println!("Received: {}", buffer);
                }
            }
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write_all(input.as_bytes()).unwrap();
        //println!("You: {}", input);
    }
}
