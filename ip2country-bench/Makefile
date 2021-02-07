build-docker:
	docker build -t iai -f Dockerfile .

bench:
	docker run -it --privileged -v "$(shell pwd)/..":/project iai \
		/bin/bash -c "cd /project/ip2country-bench && cargo bench"