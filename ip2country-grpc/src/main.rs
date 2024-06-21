//TODO: exception needed for tokio depending on old windows-sys (0.48)
#![allow(clippy::multiple_crate_versions)]

use std::net::IpAddr;
use std::sync::Arc;

use ip2c::ip_lookup_server::{IpLookup, IpLookupServer};
use ip2c::{LookupRequest, LookupResponse};
use ip2country::AsnDB;
use tonic::{transport::Server, Request, Response, Status};

pub mod ip2c {
    #![allow(
        clippy::missing_const_for_fn,
        clippy::default_trait_access,
        clippy::wildcard_imports,
        clippy::similar_names,
        clippy::future_not_send,
        clippy::missing_errors_doc
    )]
    tonic::include_proto!("ip2c");
}

pub struct LookupServer {
    db: Arc<AsnDB>,
}

#[tonic::async_trait]
impl IpLookup for LookupServer {
    async fn send(
        &self,
        request: Request<LookupRequest>,
    ) -> Result<Response<LookupResponse>, Status> {
        let country = request
            .get_ref()
            .ip
            .parse::<IpAddr>()
            .ok()
            .and_then(|ip| self.db.lookup_str(ip));

        Ok(Response::new(LookupResponse { country }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Arc::new(
        AsnDB::default()
            .load_ipv4("geo-whois-asn-country-ipv4-num.csv")?
            .load_ipv6("geo-whois-asn-country-ipv6-num.csv")?,
    );

    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();

    // creating a service
    let server = LookupServer { db };
    println!("Server listening on {addr}");

    // adding our service to our server.
    Server::builder()
        .add_service(IpLookupServer::new(server))
        .serve(addr)
        .await?;
    Ok(())
}
