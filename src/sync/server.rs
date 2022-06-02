use futures_util::future;
use http::{header, response::Builder as ResponseBuilder, StatusCode};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use hyper_staticfile::Static;
use std::{
    convert::Infallible,
    io::Error as IoError,
    net::{IpAddr, Ipv4Addr},
    net::{Ipv6Addr, SocketAddr},
    path::Path,
};

#[derive(Debug)]
pub struct Addr {
    v4: [u8; 4],
    v6: Option<[u16; 8]>,
    port: u16,
}

pub async fn run_server() {
    let addr = Addr {
        v4: [127, 0, 0, 1],
        v6: None,
        port: 3000,
    };
    let addr = SocketAddr::new(IpAddr::V4((&addr).into()), addr.port);
    let static_ = Static::new(Path::new("/root/repo/Rust/temo/"));

    let make_service = make_service_fn(|_| {
        let static_ = static_.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, static_.clone())))
    });

    let server = hyper::Server::bind(&addr).serve(make_service);

    eprintln!("Doc server running on http://{}/", addr);
    server.await.expect("Server failed");
}

async fn handle_request<B>(req: Request<B>, static_: Static) -> Result<Response<Body>, IoError> {
    if req.uri().path() == "/" {
        let res = ResponseBuilder::new()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header(header::LOCATION, "/")
            .body(Body::empty())
            .expect("unable to build response");
        Ok(res)
    } else {
        static_.clone().serve(req).await
    }
}

impl From<&Addr> for Ipv4Addr {
    fn from(addr: &Addr) -> Self {
        Ipv4Addr::new(addr.v4[0], addr.v4[1], addr.v4[2], addr.v4[3])
    }
}

impl TryFrom<&Addr> for Ipv6Addr {
    type Error = ();
    fn try_from(addr: &Addr) -> Result<Self, Self::Error> {
        if let Some(v6) = addr.v6 {
            Ok(Ipv6Addr::new(
                v6[0], v6[1], v6[2], v6[3], v6[4], v6[5], v6[6], v6[7],
            ))
        } else {
            Err(())
        }
    }
}

// REF
// https://github.com/rust-lang/rust/issues/15641#issuecomment-312013145
