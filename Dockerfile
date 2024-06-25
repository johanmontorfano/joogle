# BUILD CONFIG
FROM rustlang/rust:nightly

WORKDIR /joogle

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build

# RUNTIME CONFIG
FROM debian:buster-slim

COPY --from=0 /app/target/release/joogle /usr/local/bin/joogle

CMD ["joogle"]
