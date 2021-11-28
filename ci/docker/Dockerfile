# build from source
FROM rust as build

COPY src src
COPY Cargo.toml .
RUN cargo install --path .
RUN ln -s /usr/local/cargo/bin/dockron dockron
# runtime
FROM gcr.io/distroless/cc

COPY --from=build /dockron /usr/local/bin/dockron

ENTRYPOINT ["dockron"]