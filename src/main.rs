use std::net::SocketAddr;
use chat::broadcast::ChatSocketServer;
use std::str::FromStr;

fn main() {
    let addr :SocketAddr = SocketAddr::from_str("127.0.0.1:9999").unwrap();

    let mut chatServ :ChatSocketServer = ChatSocketServer::create(addr);
    chatServ.listening();
}

