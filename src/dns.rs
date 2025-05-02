use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Header {
    pub fn from_bytes(buf: &[u8]) -> Option<(Header, usize)> {
        if buf.len() < 12 {
            return None;
        }
        let header = Header {
            id: u16::from_be_bytes([buf[0], buf[1]]),
            flags: u16::from_be_bytes([buf[2], buf[3]]),
            qdcount: u16::from_be_bytes([buf[4], buf[5]]),
            ancount: u16::from_be_bytes([buf[6], buf[7]]),
            nscount: u16::from_be_bytes([buf[8], buf[9]]),
            arcount: u16::from_be_bytes([buf[10], buf[11]]),
        };
        Some((header, 12))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [
            self.id.to_be_bytes(),
            self.flags.to_be_bytes(),
            self.qdcount.to_be_bytes(),
            self.ancount.to_be_bytes(),
            self.nscount.to_be_bytes(),
            self.arcount.to_be_bytes(),
        ]
            .concat()
    }
}

#[derive(Debug)]
pub struct Question {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl Question {
    pub fn from_bytes(buf: &[u8]) -> Option<(Question, usize)> {
        let mut pos = 0;
        let mut labels = Vec::new();
        while pos < buf.len() {
            let len = buf[pos] as usize;
            if len == 0 {
                pos += 1;
                break;
            }
            pos += 1;
            if pos + len > buf.len() {
                return None;
            }
            labels.push(String::from_utf8_lossy(&buf[pos..pos + len]).into_owned());
            pos += len;
        }

        if pos + 4 > buf.len() {
            return None;
        }

        let qtype = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        let qclass = u16::from_be_bytes([buf[pos + 2], buf[pos + 3]]);
        pos += 4;

        Some((
            Question {
                qname: labels.join("."),
                qtype,
                qclass,
            },
            pos,
        ))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in self.qname.split('.') {
            bytes.push(label.len() as u8);
            bytes.extend(label.as_bytes());
        }
        bytes.push(0); // null terminator
        bytes.extend(&self.qtype.to_be_bytes());
        bytes.extend(&self.qclass.to_be_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub rdata: Ipv4Addr,
}

impl ResourceRecord {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Use pointer to name (offset 12 / 0x0c = start of question name)
        bytes.extend(&[0xC0, 0x0C]);
        bytes.extend(&self.rtype.to_be_bytes());
        bytes.extend(&self.rclass.to_be_bytes());
        bytes.extend(&self.ttl.to_be_bytes());
        bytes.extend(&(4u16.to_be_bytes())); // RDLENGTH for IPv4
        bytes.extend(&self.rdata.octets());
        bytes
    }
}
