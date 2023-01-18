
pub mod broadcast {
    use std::collections::HashMap;
    use std::io::BufReader;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::io::prelude::*;

    pub struct ChatSocketServer {
        listener: TcpListener,
        clients: HashMap<String, TcpStream>
    }

    impl ChatSocketServer {
        pub fn create(addr: SocketAddr) -> ChatSocketServer {
            println!("Hello New Chat Server!!!");

            ChatSocketServer {
                listener: TcpListener::bind(addr).unwrap(),
                clients: HashMap::new(),
            }
        }

        pub fn listening(&mut self) {
            for stream in self.listener.incoming() {
                let mut stream = stream.unwrap();

                handle_connection(stream);

                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }

        pub fn add_user(&mut self, id :String) {
            let client_stream:TcpStream = match self.listener.accept() {
                Ok((sock, _addr)) => sock,
                _ => unreachable!("Not Accept Stream")
            };
            self.clients.insert(id, client_stream);
        }
    }
}
