FROM rust as build

WORKDIR /usr/src/dockron
COPY . .

RUN make install

FROM debian

COPY --from=build /usr/local/bin/dockron /usr/local/bin/dockron
