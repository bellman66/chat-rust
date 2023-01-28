extern crate core;

pub mod broadcast {
    use std::collections::HashMap;
    use std::{io, thread};
    use std::io::{BufReader, BufWriter};
    use std::net::{SocketAddr, TcpListener, TcpStream};
    use std::io::prelude::*;
    use sha1::{Sha1, Digest};
    use base64::{engine::{general_purpose}, Engine};


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
                    Ok(stream) => {
                        thread::spawn(move || Self::handle_connection(stream));
                    },
                    Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => break,
                    _ => panic!("Critical Stream Error")
                }
            }
        }

        fn handle_connection(stream: TcpStream) {
            println!("Client Server in");

            // Set Field
            let mut headers = [httparse::EMPTY_HEADER; 64];
            let mut request = httparse::Request::new(&mut headers);
            let (mut reader, mut writer) = Self::get_tcp_stream_pair(stream).expect("Not Found Reader * writer");

            // // 1. Http Parse - WebSocket의 경우 http로 Connection 요청
            let mut stream_byte: [u8; 1000] = [0; 1000];
            reader.read(&mut stream_byte).expect("Failed to Read Stream");
            request.parse(&stream_byte).expect("Failed to Parsing");

            // 2. Find Secret Key
            let secret = match request.headers.iter()
                .find(|val| val.name.eq("Sec-WebSocket-Key")) {
                Some(res) => res,
                None => panic!("Not Found Secret")
            };

            // 3. Make Sha-1
            let secret_str = String::from(std::str::from_utf8(secret.value).unwrap());
            let return_key = make_sha1secret(secret_str);

            // 4. Response
            let line_msg = Self::get_response_txt(return_key);
            writer.write(line_msg.as_bytes()).unwrap();
            writer.flush().unwrap();
            println!("response OK");

            let mut message = String::new();
            loop {
                message.clear();

                if reader.read_line(&mut message).expect("Not Read") == 0 { break; }
                println!("message received: {}", message);
            }
        }

        fn get_tcp_stream_pair(s: TcpStream) -> io::Result<(BufReader<TcpStream>, BufWriter<TcpStream>)> {
            let t = s.try_clone()?;
            Ok((BufReader::new(s), BufWriter::new(t)))
        }

        fn get_response_txt(return_secret: String) -> String {
            let mut response = String::from("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: ");
            response.push_str(&return_secret);
            response.push_str("\r\n\r\n");
            response
        }

        pub fn add_user(&mut self, id :String) {
            let client_stream:TcpStream = match self.listener.accept() {
                Ok((sock, _addr)) => sock,
                _ => unreachable!("Not Accept Stream")
            };
            self.clients.insert(id, client_stream);
        }
    }

    fn make_sha1secret(mut input_str: String) -> String {
        input_str.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

        // create a Sha1 object
        let mut hasher = Sha1::new();

        // process input message
        hasher.update(input_str.as_bytes());
        let hash_buf = hasher.finalize();

        // encode
        let mut result = String::new();
        general_purpose::STANDARD.encode_string(hash_buf.as_slice(), &mut result);

        // return Value
        result
    }
}
