DEFAULT = docker-build

docker-build:
	docker image build --tag dockron-example:latest .
docker-run: docker-build
	docker run dockron-example
