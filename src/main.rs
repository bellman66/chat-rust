use std::net::SocketAddr;
use chat::BroadCast::ChatSocketServer;
use std::str::FromStr;

fn main() {
    let addr :SocketAddr = SocketAddr::from_str("127.0.0.1:9999").unwrap();

    let mut chat_serv :ChatSocketServer = ChatSocketServer::create(addr);
    chat_serv.listening();
}

