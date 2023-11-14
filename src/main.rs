use std::{net::TcpStream, io::Write};

use clap::{App, Arg};
use dns_resolver::resolve_dns;

fn main(){
    // Init the cli app required arg in domain-name optional is dns server ip address
    let app = App::new("dns-resolver")
        .about("Simple DNS Resolver!")
        .arg(
            Arg::with_name("dns-server")
                .short("s")
                .default_value("1.1.1.1"),
        )
        .arg(Arg::with_name("domain-name").required(true))
        .get_matches();

    // extract the raw domain name and init the domain name from ascii make sure there are only ascii allowed chars
    let domain_name_raw = app.value_of("domain-name").unwrap();
    // extract dns server ip address and parse it in to SocketAddr type
    let dns_server_raw = app.value_of("dns-server").unwrap();
    let resolved_dns_vec = resolve_dns(domain_name_raw, dns_server_raw);
    

    for dns in resolved_dns_vec {
        let mut conn = TcpStream::connect(format!("{}:80", dns.to_string())).unwrap();
        conn.write_all(b"GET / HTTP/1.0").unwrap(); // using this http version which after sending response close the connection.
        conn.write_all(b"\r\n").unwrap(); // signifies a new line
        conn.write_all(format!("Host: {}", dns.to_string()).as_bytes()).unwrap();
        conn.write_all(b"\r\n\r\n").unwrap(); // two new lines signifies a end of request
        std::io::copy(&mut conn, &mut std::io::stdout()).unwrap();
    }
}