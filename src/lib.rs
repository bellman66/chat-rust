extern crate core;

pub mod broadcast {
    type ClientMap = Arc<Mutex<HashMap<String, WebSocket<TcpStream>>>>;

    use std::collections::HashMap;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use tungstenite::{accept, Message, WebSocket};

    pub struct ChatSocketServer {
        listener: TcpListener,
        clients: ClientMap,
    }

    impl ChatSocketServer {
        pub fn create (addr: SocketAddr) -> ChatSocketServer {
            println!("Hello New Chat Server!!!");

            let tcp_listener = match TcpListener::bind(addr) {
                Ok(res) => res,
                _ => { panic!("Failed Bind TcpListener!")}
            };

            ChatSocketServer {
                listener: tcp_listener,
                clients: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn listening(&mut self) {
            let mut listener = &self.listener;
            for stream in listener.incoming() {
                match accept(stream.expect("Error to Stream")) {
                    Ok(mut stream) => {
                        println!("Client Server in");

                        // self.handle_connection(stream);
                        thread::spawn(move || Self::read_chat(stream));
                    },
                    _ => panic!("Critical Stream Error")
                }
            }
        }

        fn handle_connection (&mut self, stream: WebSocket<TcpStream>) {
            self.clients.lock().unwrap().insert(String::from("ok"), stream);
        }

        fn read_chat(mut stream: WebSocket<TcpStream>) {
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
}
