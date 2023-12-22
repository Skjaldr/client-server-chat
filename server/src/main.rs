use std::{
    net::{TcpStream, TcpListener},
    sync::mpsc::{Sender, Receiver, self},
    thread::{JoinHandle, self},
    io::{Read, Write}
};
use random_string::generate;

fn main() {
    let listener = match TcpListener::bind("0.0.0.0:7878") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to start server: {e}");
            return;
        }
    };

    let mut thread_vec: Vec<JoinHandle<_>> = Vec::new();
    let (tx, rx) = mpsc::channel();

    let broadcast = thread::spawn(move || {
        broadcast(rx);
    });

    for stream in listener.incoming() {
        let tx_clone = tx.clone();
        match stream {
            Ok(stream) => {
                let client_id = gen_id();

                let stream_clone = match stream.try_clone() {
                    Ok(stream_clone) => stream_clone,
                    Err(e) => {
                        eprintln!("failed to clone stream: {e}");
                        continue;
                    }
                };

                if let Err(e) = tx_clone.send(ServerMessages::NewClient(stream_clone, client_id.clone())) {
                    eprintln!("Failed to send client information over the channel: {e}");
                    continue;
                }

                let handle = thread::spawn(move || {
                    handle_clients(stream, tx_clone, client_id);
                });
                thread_vec.push(handle);
            }
            Err(e) => eprintln!("Failed to match stream: {e}"),

        }
    }
}

struct ClientMsgId {
    message: String,
    client_id: String,
}

enum ServerMessages {
    NewClient(TcpStream, String),
    Messages(ClientMsgId),
}

fn handle_clients(mut stream: TcpStream, tx: Sender<ServerMessages>, client_id: String) {
    loop {
        let mut buf = [0; 4];
        if let Ok(_) = stream.read_exact(&mut buf) {
            let msg_size = u32::from_be_bytes(buf) as usize;
            let mut msg_buf = vec![0; msg_size];

            if let Ok(_) = stream.read_exact(&mut msg_buf) {
                let message = String::from_utf8_lossy(&msg_buf).to_string();

                if let Err(e) = tx.send(ServerMessages::Messages(
                    ClientMsgId { message, client_id: client_id.clone() })) {
                        eprintln!("Failed to send messages to clients: {e}");
                        continue;
                };
            }
        }
    }
}

fn broadcast(rx: Receiver<ServerMessages>) {
    let mut clients: Vec<(TcpStream, String)> = Vec::new();
    loop {
        while let Ok(new_client) = rx.recv() {
            match new_client {
                ServerMessages::NewClient(client, id) => {
                    clients.push((client, id));
                }
                ServerMessages::Messages(message) => {
                    let message_buf = message.message.as_bytes();
                    let msg_size = buf_size.len() as u32;
                    let prefix_size = msg_size.to_be_bytes();
                    for (client, id) in clients.iter_mut() {
                        if message.client_id != *id {
                            // broadcast dah messuge!

                            if let Err(e) = client.write_all(&prefix_size) {
                                eprintln!("Failed to write prefix size to stream: {e}");
                                continue;
                            }
                            if let Err(e) = client.write_all(message_buf) {
                                eprintln!("Failed to write message to stream: {e}");
                                continue;
                            }
                            if let Err(e) = client.flush() {
                                eprintln!("Failed to flush the stream buffer: {e}");
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn gen_id() -> String {
    let charset = "AbCdEfGhIjKlMnOpQrStUvWxYz!1@2#3$4%5^6&7*8(9)0";
    generate(16, charset)
}
