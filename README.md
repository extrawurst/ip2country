# ip2country

uses free (CC0) APNs lookup [tables](https://github.com/sapics/ip-location-db) for ip to country lookup based on [OpenGeoFeed.com](https://opengeofeed.org).

find the docker image on [docker hub](https://hub.docker.com/repository/docker/extrawurst/ip2country).

# features

* lightweight and fast using rust
* supports **ipv4** and **ipv6**
* free APNs tables, no license mess
* nightly updated with fresh APNs tables

# example

```
docker run -d --rm --name ip2country -p 5000:5000 extrawurst/ip2country:latest

curl http://0.0.0.0:5000/2a00:1450:4005:800::200e
US

curl http://0.0.0.0:5000/172.217.16.78
US

docker kill ip2country
```
