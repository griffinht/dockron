DEFAULT = build

build:
	cargo build
install: build
	ln -s $(shell cd ./target/debug/; pwd)/dockron /usr/local/bin/dockron
uninstall:
	rm /usr/local/bin/dockron
clean:
	rm -rf ./target/
docker-build:
	docker build . --tag dockron:latest