#![allow(clippy::multiple_crate_versions)]

use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming as IncomingBody},
    server::conn::http1,
    service::service_fn,
    Method, Request, Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use ip2country::AsnDB;
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tokio::{net::TcpListener, task::spawn_blocking};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

async fn ip_lookup(uri: String, db: Arc<AsnDB>) -> Result<Response<Full<Bytes>>> {
    Ok(spawn_blocking(move || {
        if uri.len() >= 8 {
            if let Ok(ip) = uri[1..uri.len()].parse::<IpAddr>() {
                log::info!("lookup: {}", ip);
                if let Some(code) = db.lookup_str(ip) {
                    return Response::new(code.into());
                }
                log::warn!("ip lookup failed: {}", ip);
                return Response::new("".into());
            }
        }

        bad_request()
    })
    .await?)
}

fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("".into())
        .unwrap()
}

fn bad_request() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("".into())
        .unwrap()
}

async fn routing(req: Request<IncomingBody>, db: Arc<AsnDB>) -> Result<Response<Full<Bytes>>> {
    match (req.method(), req.uri().path()) {
        // (&Method::GET, "/myip") => Ok(Response::new(INDEX.into())),
        (&Method::GET, uri) => ip_lookup(uri.to_string(), db.clone()).await,

        _ => Ok(not_found()),
    }
}

fn get_port() -> u16 {
    if let Ok(env) = std::env::var("PORT") {
        if let Ok(port) = env.parse::<u16>() {
            return port;
        }
    }

    5000
}

#[tokio::main]
pub async fn main() -> Result<()> {
    pretty_env_logger::init();

    let db = Arc::new(
        AsnDB::default()
            .load_ipv4("geo-whois-asn-country-ipv4-num.csv")?
            .load_ipv6("geo-whois-asn-country-ipv6-num.csv")?,
    );

    println!(
        "google.com: {:?}",
        db.lookup(String::from("172.217.16.78").parse().unwrap())
    );

    let addr: SocketAddr = ([0, 0, 0, 0], get_port()).into();

    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{addr}");

    loop {
        let (tcp, _) = listener.accept().await?;

        let io = TokioIo::new(tcp);

        let db_arc = Arc::clone(&db);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| routing(req, db_arc.clone()));

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                println!("Error serving connection: {err:?}");
            }
        });
    }
}
