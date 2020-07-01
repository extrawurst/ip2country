
docker-build:
	docker build -t ipsrv -f Dockerfile .

docker-run:
	docker run -it -p 3000:3000 ipsrv