use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tiny_http::{Response, Server, StatusCode};

fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(..) => 8000,
    };
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    let server = Server::http(address).unwrap();

    let response = Response::empty(StatusCode(200));
    for req in server.incoming_requests() {
        let _ = req.respond(response.clone());
    }
}
