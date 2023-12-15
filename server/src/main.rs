use std::{
    thread,
    io::{Read, Write},
    net::{TcpStream, TcpListener}, sync::mpsc::{Receiver, Sender, self}, time::Duration,
};

fn broadcast_message(msg_rx: Receiver<String>, client_rx: Receiver<TcpStream>) {
    let mut clients: Vec<TcpStream> = Vec::new();
    loop {
        while let Ok(new_client) = client_rx.try_recv() {
            clients.push(new_client);
        }



        while let Ok(msg) = msg_rx.try_recv() {
            clients.retain(|mut client| {
                client.write_all(msg.as_bytes()).is_ok()
            });
        }

        // while let Ok(msg) = msg_rx.try_recv() {
        //     for client in clients.iter_mut() {
        //         client.write_all(msg.as_bytes()).is_ok();

        //     }
        // }
        thread::sleep(Duration::from_millis(1));
    }
}

fn handle_clients(mut stream: TcpStream, tx: Sender<String>) {
    // use tx to send the input from user

    loop {
        let mut buf = [0; 1024];
        let read_bytes = stream.read(&mut buf).unwrap();

        if read_bytes == 0 {
            break;
        }

        //stream.write(&buf[..read_bytes]).unwrap();
        let msg = String::from_utf8_lossy(&buf[..read_bytes]).to_string();
        tx.send(msg).unwrap();
    }
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
        let stream = stream.unwrap();
        let tx_clone: Sender<String> = tx.clone();
        client_tx.send(stream.try_clone().unwrap()).unwrap();

        // connection thread handler
        let handle = thread::spawn(move || {
            let _ = handle_clients(stream, tx_clone);

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
