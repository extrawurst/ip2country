#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::panic)]
#![allow(clippy::multiple_crate_versions)]
//TODO: remove in 1.45
#![allow(clippy::used_underscore_binding)]

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use ip2country::AsnDB;
use std::{net::Ipv4Addr, sync::Arc};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

async fn ip_lookup(uri: String, db: &Arc<AsnDB>) -> Result<Response<Body>> {
    if uri.len() >= 8 {
        if let Ok(ip) = uri[1..uri.len()].parse::<Ipv4Addr>() {
            if let Some(code) = db.lookup_str(ip) {
                return Ok(Response::new(code.into()));
            } else {
                log::warn!("ip lookup failed: {}", ip);
                return Ok(Response::new("".into()));
            }
        }
    }

    Ok(bad_request())
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("".into())
        .unwrap()
}

fn bad_request() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("".into())
        .unwrap()
}

async fn routing(req: Request<Body>, db: Arc<AsnDB>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        // (&Method::GET, "/myip") => Ok(Response::new(INDEX.into())),
        (&Method::GET, uri) => {
            let uri = String::from(uri);
            ip_lookup(uri, &db).await
        }

        _ => Ok(not_found()),
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    pretty_env_logger::init();

    let db = Arc::new(AsnDB::load("geo-whois-asn-country-ipv4-num.csv"));

    let db_arc = Arc::clone(&db);

    let service = make_service_fn(move |_| {
        let db = db_arc.clone();
        async { Ok::<_, GenericError>(service_fn(move |req| routing(req, db.clone()))) }
    });

    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
