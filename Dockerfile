FROM rust:slim-buster as builder

RUN USER=root cargo new --bin teaboard
WORKDIR /teaboard

COPY Cargo.lock Cargo.toml ./

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/teaboard*
RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /teaboard/target/release/teaboard /bin/teaboard

ARG PORT=7158

EXPOSE $PORT
ENV ROCKET_ADDRESS 0.0.0.0
ENV ROCKET_PORT $PORT

ENV RUST_LOG debug

CMD ["/bin/teaboard"]
