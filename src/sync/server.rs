use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{
    convert::Infallible,
    net::SocketAddr,
    net::{IpAddr, Ipv4Addr},
};




pub async fn run_server() {
    //pub async fn run_server(ip: &IpAddr, port: u32) {
    // We'll bind to 127.0.0.1:3000
    let port = 3000;
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("{}:{}", addr, port);
    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

pub async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

mod test {

    #[test]
    fn t_run_server() {
        let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    }
}

// REF
// https://github.com/rust-lang/rust/issues/15641#issuecomment-312013145
