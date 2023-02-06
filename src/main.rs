use std::net::{SocketAddr, TcpStream};
use chat::BroadCast::{ChatSocketServer, ClientStation};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use tungstenite::WebSocket;

fn main() {
    let addr :SocketAddr = SocketAddr::from_str("127.0.0.1:9999").unwrap();
    let (sender, receiver) = mpsc::channel();

    let mut chat_serv: ChatSocketServer = ChatSocketServer::create(addr, sender);

    thread::spawn(move || {
        let mut station = ClientStation::create(receiver);
        station.read_station();
    });

    chat_serv.listening();
}

