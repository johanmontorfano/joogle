# BUILD CONFIG
FROM rustlang/rust:nightly

WORKDIR /joogle

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN rustup component add rustfmt
RUN cargo build

# RUNTIME CONFIG
FROM debian:latest

RUN apt-get update 
RUN DEBIAN_FRONTEND=noninteractive apt-get install --yes libcurl4 
RUN DEBIAN_FRONTEND=noninteractive apt-get install --yes libc-bin
RUN apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY --from=0 /joogle/target/debug/joogle /usr/local/bin/joogle

CMD ["joogle"]
EXPOSE 8000
