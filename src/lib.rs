extern crate core;

pub mod broadcast {
    use std::collections::HashMap;
    use std::fs::read;
    use std::io;
    use std::io::BufReader;
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::io::prelude::*;

    pub struct ClientRequestInfo {
        host: String,
        connection : String,
        upgrade: String,
        origin: String,
        websocket_key: [u8]
    }

    impl ClientRequestInfo {
        pub fn create() {

        }
    }


    pub struct ChatSocketServer {
        listener: TcpListener,
        clients: HashMap<String, TcpStream>,
    }

    impl ChatSocketServer {
        pub fn create(addr: SocketAddr) -> ChatSocketServer {
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

        pub fn listening(&mut self) {
            for stream in self.listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        println!("Client Server in");

                        // Set Field
                        let mut headers = [httparse::EMPTY_HEADER; 64];
                        let mut parser =  httparse::Request::new(&mut headers);

                        // // 1. Http Parse
                        // // ! WebSocket의 경우 http로 Connection 요청
                        let mut stream_byte :[u8; 1000] = [0;1000];
                        stream.read(&mut stream_byte).expect("Failed to Read Stream");

                        // let readline = std::str::from_utf8(&stream_byte).expect("Failed to Convert Stream utf-8");
                        // println!("{}", readline);

                        parser.parse(&stream_byte);

                        // 2. Connection 결과값 반환.
                        for head in parser.headers.iter() {
                            let val = std::str::from_utf8(head.value).unwrap();
                            println!("{} / {}", head.name, val);
                        }

                        // 3. Response
                        ChatSocketServer::response_client(stream);
                    },
                    Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => break,
                    _ => panic!("Critical Stream Error")
                }
            }
        }

        fn response_client(mut stream: TcpStream) {
            let response = "HTTP/1.1 101 Switching Protocols
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            println!("response OK")
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
