LABEL maintainer="extrawurst"
FROM rust AS builder
WORKDIR ipsrv
ADD ip2country ./ip2country
ADD ip2country-service ./ip2country-service
ADD Cargo.toml ./Cargo.toml
ADD Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN cp ./target/release/ip2country-service ./target/ip2country

FROM ubuntu
RUN apt-get update && apt-get install -y openssl
WORKDIR ipsrv
ADD ip2country-service/geo-whois-asn-country-ipv4-num.csv ./
COPY --from=builder /ipsrv/target/ip2country ./
CMD ["./ip2country"]