# builder image
FROM rust:latest as builder
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/n2i-power
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc cargo install --path . --target=x86_64-unknown-linux-musl

# generate clean, final image for end users
FROM alpine:latest
COPY --from=builder /usr/src/n2i-power/target/release/n2i-power .

# executable
ENTRYPOINT [ "./n2i-power" ]

# Build
# $ docker build . -t n2i-power:latest

# Run
# $ docker run --restart=always -d --name n2i-power n2i-power:latest
