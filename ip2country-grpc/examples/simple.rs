use ip2c::{ip_lookup_client::IpLookupClient, LookupRequest};

pub mod ip2c {
    tonic::include_proto!("ip2c");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    // creating gRPC client from channel
    let mut client = IpLookupClient::new(channel);

    // creating a new Request
    let request = tonic::Request::new(LookupRequest {
        ip: String::from("8.8.8.8"),
    });

    // sending request and waiting for response
    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
