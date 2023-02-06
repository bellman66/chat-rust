extern crate core;

pub mod BroadCast {
    type ClientMap = Arc<Mutex<HashMap<String, WebSocket<TcpStream>>>>;
    type ReceiveArc = Arc<Mutex<Receiver<WebSocket<TcpStream>>>>;

    use std::collections::HashMap;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::sync::{Arc, mpsc, Mutex};
    use std::{thread};
    use std::fs::read;
    use std::sync::mpsc::{Receiver, Sender};
    use tungstenite::{accept, Message, WebSocket};

    pub struct ClientStation {
        cnt: i32,
        sockets: ClientMap,
        receiver: ReceiveArc
    }

    impl ClientStation {
        pub fn create(_receiver: Receiver<WebSocket<TcpStream>>) -> ClientStation {
            ClientStation {
                cnt: 0,
                sockets: Arc::new(Mutex::new(HashMap::new())),
                receiver: Arc::new(Mutex::new(_receiver))
            }
        }

        pub fn read_station(&mut self) {
            let ref arc = self.receiver.lock().unwrap();

            while let Ok(mut data) = arc.recv() {
                thread::spawn(move || {
                    let mut guard = &self.sockets.lock().unwrap();
                    guard.insert(String::from("test"), data);

                    let mut readSocket = guard.get_mut("test").unwrap();
                    Self::read_chat(readSocket);
                });
            }
        }

        fn read_chat(stream: &mut WebSocket<TcpStream>) {
            loop {
                match stream.read_message() {
                    Ok(msg) => {
                        match msg {
                            Message::Text(_) => {
                                let string = msg.to_string();
                                let str = string.as_str();

                                let mut return_msg = String::from("return : ");
                                return_msg.push_str(str);
                                stream.write_message(Message::text(return_msg));
                            }
                            Message::Binary(_) => { println!("{}", msg.to_string()); }
                            Message::Ping(_) => {}
                            Message::Pong(_) => {}
                            Message::Close(_) => {}
                            Message::Frame(_) => {}
                        }
                    },
                    Err(_) => {panic!("Failed to Read Msg")}
                }
            }
        }
    }

    pub struct ChatSocketServer {
        listener: TcpListener,
        sender: Sender<WebSocket<TcpStream>>
    }

    impl ChatSocketServer {
        pub fn create (aAddr: SocketAddr, aSender: Sender<WebSocket<TcpStream>>) -> ChatSocketServer {
            println!("Hello New Chat Server!!!");

            ChatSocketServer {
                listener: match TcpListener::bind(aAddr) {
                    Ok(res) => res,
                    _ => { panic!("Failed Bind TcpListener!")}
                },
                sender: aSender
            }
        }

        pub fn listening(&mut self) {
            for stream in self.listener.incoming() {
                match accept(stream.expect("Error to Stream")) {
                    Ok(mut stream) => {
                        println!("Client Server in");

                        // Process : Call Mpsc & Store Stream
                        self.sender.send(stream).unwrap();
                    },
                    _ => panic!("Critical Stream Error")
                }
            }
        }
    }
}
