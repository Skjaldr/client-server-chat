use std::{
    thread,
    net::TcpStream,
    io::{BufRead, BufReader, self, Write,},
};

/*   THIS IS THE CLIENT */


// STEPS
// create the stream - connect the client to 127.0.0.1:7878
// create an input variable to store the strings, create a new string
// accept input from the user using io::stdin() and assign it to the input var
// write to the stream using the input variable that contains user input.
// create a reader and init to a new BufReader utilizing the stream.
// create a buffer - a vector of u8's.
// have the reader read the lines coming in until return is pressed b'\n'
// print out the value that the server received.

/* Steps for suggested creation of client for chat server
1. Create stream
2. stream clone - try_clone()
3. create thread for reading input
    1. create buf reader with stream clone
    2. loop
        1. create new String named buffer
        2. if let Ok(read_bytes) = reader.read_line
            1. if read_bytes > 0 -> println! output for received messages

4. Create Loop
    1. In loop create string buffer
    2. take in stdio = io::stdin().read_line(&mut input);
    3. write to the buffer using stream. stream.write_all(input.as_bytes());
 */


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let stream_clone = stream.try_clone().unwrap();

    thread::spawn(move || {
        let mut reader = BufReader::new(&stream_clone);
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

    }
}
