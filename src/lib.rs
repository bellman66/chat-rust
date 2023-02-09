extern crate core;

pub mod BroadCast {
    type ClientMap = Arc<Mutex<HashMap<String, RefCell<WebSocket<TcpStream>>>>>;

    use std::collections::HashMap;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::sync::{Arc, mpsc, Mutex};
    use std::{thread};
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::{RefCell, RefMut};
    use std::fs::read;
    use std::rc::Rc;
    use std::sync::mpsc::{Receiver, Sender};
    use tungstenite::{accept, Message, WebSocket};

    pub struct ClientStation {
        cnt: i32,
        sockets: ClientMap,
        receiver: Receiver<WebSocket<TcpStream>>
    }

    impl ClientStation {
        pub fn create(_receiver: Receiver<WebSocket<TcpStream>>) -> ClientStation {
            ClientStation {
                cnt: 0,
                sockets: Arc::new(Mutex::new(HashMap::new())),
                receiver: _receiver
            }
        }

        pub fn read_station(&mut self) {
            while let Ok(mut sock) = self.receiver.recv() {
                let blockMap = self.sockets.clone();

                thread::spawn(move || {
                    let mut mutexSocket = blockMap.lock().unwrap();
                    mutexSocket.insert(String::from("test"), RefCell::new(sock));

                    let option = mutexSocket.get("test").unwrap();
                    let ref_mut = option.borrow_mut();
                    Self::read_chat(ref_mut);
                });
            };
        }

        fn read_chat(mut stream: RefMut<WebSocket<TcpStream>>) {
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
