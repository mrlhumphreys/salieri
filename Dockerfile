# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# multistate build first step
FROM rust:1.61.0 as builder

RUN USER=root cargo new --bin salieri 
WORKDIR /salieri

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# build to cache dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/salieri*
RUN cargo build --release 

# multistage build second step
FROM rust:1.61.0-slim-buster

COPY --from=builder /salieri/target/release/salieri .

CMD ["./salieri"]

