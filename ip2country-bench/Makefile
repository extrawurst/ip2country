build-docker:
	docker build -t iai -f Dockerfile .

bench:
	docker run -it -v "$(shell pwd)/..":/project iai \
		/bin/bash -c "cd /project/ip2country-bench && make bench-inside-docker"

bench-inside-docker:
	IAI_ALLOW_ASLR=true cargo bench