# ip2country

find it on [docker hub](https://hub.docker.com/repository/docker/extrawurst/ip2country)

# features

* lightweight and fast
* free and no license mess (CC0)
* nightly updated with fresh APNs tables

# example

```
docker run -d --rm --name ip2country -p 5000:5000 extrawurst/ip2country:latest
curl http://0.0.0.0:5000/1.2.3.4
AU
docker kill ip2country
```
