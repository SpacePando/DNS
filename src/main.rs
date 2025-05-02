use std::net::UdpSocket;
mod dns;
use dns::{Header, Question, ResourceRecord};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:5300").expect("Unable to bind to UDP port 53");
    println!("DNS server listening on port 53");

    loop {
        let mut buf = [0u8; 512];
        let (len, addr) = match socket.recv_from(&mut buf) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("recv_from failed: {}", e);
                continue;
            }
        };

        let req = &buf[..len];
        let (header, hlen) = match Header::from_bytes(req) {
            Some(res) => res,
            None => continue,
        };

        if header.qdcount != 1 {
            eprintln!("Unsupported QDCOUNT: {}", header.qdcount);
            continue;
        }

        let (question, qlen) = match Question::from_bytes(&req[hlen..]) {
            Some(res) => res,
            None => continue,
        };

        println!("Received query: {:?}", question.qname);

        // Build response header
        let response_header = Header {
            id: header.id,
            flags: 0x8180, // Standard query response, no error
            qdcount: 1,
            ancount: 1,
            nscount: 0,
            arcount: 0,
        };

        // Hardcoded A record: 1.2.3.4
        let answer = ResourceRecord {
            name: question.qname.clone(),
            rtype: 1,
            rclass: 1,
            ttl: 60,
            rdata: "1.2.3.4".parse().unwrap(),
        };

        let mut response = Vec::new();
        response.extend(response_header.to_bytes());
        response.extend(question.to_bytes());
        response.extend(answer.to_bytes());

        if let Err(e) = socket.send_to(&response, addr) {
            eprintln!("send_to failed: {}", e);
        }
    }
}
