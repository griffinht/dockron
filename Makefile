DEFAULT = build

build:
	cargo build
install: build
	sudo ln -s $(shell cd ./target/debug/; pwd)/dockron /usr/local/bin/dockron
uninstall:
	sudo rm /usr/local/bin/dockron
clean:
	rm -rf ./target/