
BASE_URL=https://cdn.jsdelivr.net/npm/@ip-location-db/geo-whois-asn-country
IP4_FILE=geo-whois-asn-country-ipv4-num.csv
IP6_FILE=geo-whois-asn-country-ipv6-num.csv

update:
	curl "${BASE_URL}/${IP4_FILE}" > ip2country-service/${IP4_FILE}
	curl "${BASE_URL}/${IP6_FILE}" > ip2country-service/${IP6_FILE}

docker-build:
	docker build -t ipsrv -f Dockerfile .

docker-run:
	docker run -it -p 3000:3000 ipsrv