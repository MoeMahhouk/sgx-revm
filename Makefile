build:
	sudo docker build -t sgx-revm-app .

run:
	sudo docker run --privileged --device /dev/sgx --rm -it -p 7878:7878 --name sgx-revm-container sgx-revm-app

clean-images:
	echo y | docker image prune

clean-all: 
	echo y | docker system prune

.PHONY: build run clean