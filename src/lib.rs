extern crate core;

pub mod BroadCast {
    type StreamMutex = Arc<Mutex<Client>>;
    type ClientMap = Arc<Mutex<HashMap<String, StreamMutex>>>;

    use std::collections::HashMap;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::sync::{Arc, mpsc, Mutex, MutexGuard};
    use std::{thread};
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::{RefCell, RefMut};
    use std::fs::read;
    use std::rc::Rc;
    use std::sync::mpsc::{Receiver, Sender};
    use tungstenite::{accept, client, Message, WebSocket};
    use uuid::Uuid;

    pub struct Client {
        id: String,
        socket: WebSocket<TcpStream>
    }

    pub struct ClientStation {
        cnt: i32,
        clients: ClientMap,
        receiver: Receiver<Client>
    }

    impl ClientStation {
        pub fn create(_receiver: Receiver<Client>) -> ClientStation {
            ClientStation {
                cnt: 0,
                clients: Arc::new(Mutex::new(HashMap::new())),
                receiver: _receiver
            }
        }

        pub fn read_station(&mut self) {
            while let Ok(mut clintInfo) = self.receiver.recv() {
                let stream_mutex = self.clients.clone();

                thread::spawn(move || {
                    let id = clintInfo.id.clone();
                    let id_str = id.clone();

                    let mut clients_mutex = stream_mutex.lock().unwrap();
                    clients_mutex.insert(id, Arc::new(Mutex::new(clintInfo)));

                    let stream_arc = clients_mutex.get(&id_str).unwrap();
                    let socket_mutex = stream_arc.clone();
                    thread::spawn(move || {
                        let mut client_guard = socket_mutex.lock().unwrap();
                        Self::read_chat(client_guard);
                    });
                });
            };
        }

        fn read_chat(mut client: MutexGuard<Client>) {
            let client_id = client.id.clone();
            let ref mut stream = client.socket;
            loop {
                match stream.read_message() {
                    Ok(msg) => {
                        match msg {
                            Message::Text(_) => {
                                let msg_str = msg.to_string();
                                let input_msg = msg_str.as_str();

                                // Make msg
                                let mut return_msg= Self::get_format_msg(&client_id, input_msg);
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

        fn get_format_msg(client_id: &str, input_msg: &str) -> String {
            let mut return_msg = String::from(client_id);
            return_msg.push_str(" : ");
            return_msg.push_str(input_msg);
            return_msg
        }
    }

    pub struct ChatSocketServer {
        listener: TcpListener,
        sender: Sender<Client>
    }

    impl ChatSocketServer {
        pub fn create (aAddr: SocketAddr, aSender: Sender<Client>) -> ChatSocketServer {
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

                        // Make UUID
                        let id_str = Uuid::new_v4().to_string();

                        // Process : Call Mpsc & Store Stream
                        self.sender.send(Client {id: id_str, socket: stream}).unwrap();
                    },
                    _ => panic!("Critical Stream Error")
                }
            }
        }
    }
}
