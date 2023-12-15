use std::{
    thread,
    io::{Read, Write},
    net::{TcpStream, TcpListener}, sync::mpsc::{Receiver, Sender, self}, time::Duration,
};
use random_string::generate;

struct ClientMsg {
    message: String,
    client_id: String,
}

fn broadcast_message(msg_rx: Receiver<ClientMsg>, client_rx: Receiver<(TcpStream, String)>) {
    let mut clients: Vec<(TcpStream, String)> = Vec::new();
    loop {

        while let Ok((new_client, id)) = client_rx.try_recv() {
            clients.push((new_client, id));
        }


        while let Ok(msg) = msg_rx.try_recv() {
            for (client, id) in clients.iter_mut() {
                if msg.client_id != *id {
                    let _ = client.write_all(msg.message.as_bytes()).unwrap();
                }
            }
        }

        thread::sleep(Duration::from_millis(1));
    }
}

fn handle_clients(mut stream: TcpStream, tx: Sender<ClientMsg>, client_id: String) {
    // use tx to send the input from user

    loop {
        let mut buf = [0; 1024];
        let read_bytes = stream.read(&mut buf).unwrap();

        if read_bytes == 0 {
            break;
        }

        let client_message = ClientMsg {
            message: String::from_utf8_lossy(&buf[..read_bytes]).to_string(),
            client_id: client_id.clone()
        };
        tx.send(client_message).unwrap();

        //stream.write(&buf[..read_bytes]).unwrap();
        // let msg = String::from_utf8_lossy(&buf[..read_bytes]).to_string();
    }
}

fn generate_client_id() -> String {
    let charset = "aABbcCDdeEfFGgHhiIJjkKlLmMNnoOPpqQrRsStTuUvVWwxXyYzZ1!2@3#4$5%6^7&8*9(";
    let cid = generate(16, charset);
    cid
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let (tx, rx) = mpsc::channel();
    let (client_tx, client_rx)= mpsc::channel();
    let mut thread_vec: Vec<thread::JoinHandle<_>> = Vec::new();

    let broadcast = thread::spawn(move || {
        broadcast_message(rx, client_rx);
    });

    for stream in listener.incoming() {
        let client_id = generate_client_id();
        let stream = stream.unwrap();
        let tx_clone: Sender<ClientMsg> = tx.clone();
        client_tx.send((stream.try_clone().unwrap(), client_id.clone())).unwrap();

        // connection thread handler
        let handle = thread::spawn(move || {
            let _ = handle_clients(stream, tx_clone, client_id);

        });
        // join handle threads
        thread_vec.push(handle);

    }

    // for handle in thread_vec {
    //     handle.join().unwrap();
    // }
    // broadcast.join().unwrap();
    // join broadcast threads
}
