
BASE_URL=https://cdn.jsdelivr.net/npm/@ip-location-db/geo-whois-asn-country
IP4_FILE=geo-whois-asn-country-ipv4-num.csv
IP6_FILE=geo-whois-asn-country-ipv6-num.csv

download:
	curl "${BASE_URL}/${IP4_FILE}" > ip2country-service/${IP4_FILE}
	curl "${BASE_URL}/${IP6_FILE}" > ip2country-service/${IP6_FILE}
	cp ip2country-service/${IP4_FILE} ip2country-grpc/${IP4_FILE}
	cp ip2country-service/${IP6_FILE} ip2country-grpc/${IP6_FILE}

docker-local:
	docker build -t extrawurst/ip2country:latest -f Dockerfile.local .

docker-run:
	docker run -it -p 5000:5000 extrawurst/ip2country:latest

test:
	# run this once the container runs locally (see above)
	xh 0.0.0.0:5000/172.217.16.78

check:
	cargo clippy --workspace
