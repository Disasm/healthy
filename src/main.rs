use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tiny_http::{Response, Server, StatusCode};

fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(..) => 8000,
    };
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    let server = Arc::new(Server::http(address).unwrap());
    println!("Server started.");

    let server_copy = server.clone();
    ctrlc::set_handler(move || {
        println!("Received termination signal.");
        server_copy.unblock();
    })
    .expect("Error setting signal handler");

    let response = Response::empty(StatusCode(200));
    for req in server.incoming_requests() {
        let _ = req.respond(response.clone());
    }
    println!("Stopped.");
}
