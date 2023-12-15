use std::{
    io::{Result, self, Read, Write, BufReader, BufRead},
    net::{TcpListener, TcpStream},
    thread, sync::mpsc::{self, Sender},
};

/* THIS IS THE SERVER */

// main thread
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut vec_threads: Vec<thread::JoinHandle<()>> = Vec::new();
    let (tx, rx) = mpsc::channel();

    let tx_clone_1: Sender<String> = tx.clone();
    let broadcast = thread::spawn(move || {
        //active_clients(rx);
        rx.recv().unwrap();
    });


    // listening for incoming messages
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let tx_clone: Sender<String> = tx_clone_1.clone();
        let handle = thread::spawn(move || {
            handle_cli(stream, tx_clone).unwrap();
        });
        vec_threads.push(handle);
    }

    // join connected threads
    for handle in vec_threads {
        handle.join().unwrap();
    }
    broadcast.join().unwrap();

}

fn active_clients(rx: mpsc::Receiver<String>) -> Result<()> {


    Ok(())
}

fn handle_cli(mut stream: TcpStream, tx: Sender<String>) -> io::Result<()> {
    loop {
        let mut buffer = [0; 1024];
        let read_bytes = stream.read(&mut buffer).unwrap();
        if read_bytes <= 0 {
            break;
        }
        stream.write(&buffer[..read_bytes]).unwrap();
        let msg = String::from_utf8_lossy(&buffer).to_string();
        tx.send(msg).unwrap();

    }
    Ok(())
}
