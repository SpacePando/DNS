use std::net::UdpSocket;

mod dns;
use dns::Header;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:1053").expect("Could not bind to port 1053");
    let mut buf = [0; 512];

    println!("DNS server is running at port 1053");

    loop {
        let (len, addr) = socket.recv_from(&mut buf).expect("Could not receive data");
        let header = Header::from_bytes(&buf[..len]).expect("Could not parse DNS header");
        println!("Received query from {} {:?}", addr, header);
    }
}