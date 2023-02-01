extern crate core;

pub mod broadcast {
    use std::collections::HashMap;
    use std::{thread};
    use std::borrow::{BorrowMut};
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use tungstenite::{accept, Message, WebSocket};

    pub struct ChatSocketServer <'a> {
        listener: TcpListener,
        clients: HashMap<String, &'a WebSocket<TcpStream>>,
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
                clients: HashMap::new(),
            }
        }

        pub fn listening(self) {
            for stream in self.listener.incoming() {
                match accept(stream.expect("Error to Stream")) {
                    Ok(mut stream) => {
                        println!("Client Server in");

                        self.handle_connection(stream.borrow_mut());
                        thread::spawn(move || Self::read_chat(stream));
                    },
                    _ => panic!("Critical Stream Error")
                }
            }
        }

        fn handle_connection(mut self, stream: &'a mut WebSocket<TcpStream>) {
            self.clients.insert(String::from("ok"), stream);
        }

        fn read_chat(mut stream: WebSocket<TcpStream>) {
            loop {
                match stream.read_message() {
                    Ok(msg) => {
                        match msg {
                            Message::Text(_) => { println!("{}", msg.to_string()); }
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
