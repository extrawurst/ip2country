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

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("ip2c_descriptor");
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

        //TODO: use tracing
        // tracing::info!("lookup: {} -> {country:?}", request.get_ref().ip,);
        println!("lookup: {} -> {country:?}", request.get_ref().ip,);

        Ok(Response::new(LookupResponse { country }))
    }
}

fn get_service_addr() -> String {
    std::env::var("SRV_ADDR").unwrap_or_else(|_| "[::1]:50051".into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Arc::new(
        AsnDB::default()
            .load_ipv4("geo-whois-asn-country-ipv4-num.csv")?
            .load_ipv6("geo-whois-asn-country-ipv6-num.csv")?,
    );

    // defining address for our service
    let addr = get_service_addr().parse().unwrap();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(ip2c::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    // creating a service
    let server = LookupServer { db };
    println!("Server listening on {addr}");

    // adding our service to our server.
    Server::builder()
        .add_service(reflection_service)
        .add_service(IpLookupServer::new(server))
        .serve(addr)
        .await?;
    Ok(())
}
