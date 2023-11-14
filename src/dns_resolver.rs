use std::{
    net::{SocketAddr, UdpSocket, IpAddr},
    time::Duration,
};

use clap::{App, Arg};
use trust_dns::{
    op::{Message, MessageType, OpCode, Query},
    rr::{Name, RecordType},
    serialize::binary::{BinEncodable, BinEncoder},
};

pub fn resolve_dns(domain_name_raw:&str, dns_server_raw:&str) -> Vec<IpAddr> {
    let domain_name = Name::from_ascii(domain_name_raw).unwrap();

    let dns_server: SocketAddr = format!("{}:53", dns_server_raw)
        .parse::<SocketAddr>()
        .expect("invalid address for dns server");

    // request as bytes create vector with capacity 512 and length 512 macro create vector with length and cap 512
    // needed for recv_from cause it check if it has enough space for the msg in vector.
    let mut request_as_bytes = Vec::<u8>::with_capacity(512);
    let mut response_as_bytes = vec![0; 512];

    // create a message for DNS resolver. Need id, message type Query, Queries can be multiple in one call, recursion if the dns server should ask other servers upstream
    let mut message = Message::new();
    message
        .set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);
    // convert message into raw bytes
    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    message.emit(&mut encoder).unwrap();

    // listen on all addresses on random port
    let localhost = UdpSocket::bind("0.0.0.0:0").expect("cannot bind to local socket!");

    let timout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let _amt = localhost
        .send_to(&request_as_bytes, dns_server)
        .expect("socket misconfigured");

    let (_amt, _remote) = localhost
        .recv_from(&mut response_as_bytes)
        .expect("timeout reached");

    let dns_message = Message::from_vec(&response_as_bytes).expect("unable to parse response");
    let mut resolved_dns: Vec<IpAddr> = Vec::new();
    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            let ip_add = resource.to_ip_addr().expect("wrong ip address received");
            resolved_dns.push(ip_add);
        }
    }
    return resolved_dns;
}
