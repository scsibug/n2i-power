# builder image
FROM rust:latest as builder
RUN apt-get update
WORKDIR /usr/src/n2i-power
COPY . .
RUN cargo install --path .

# generate clean, final image for end users
FROM debian:stable-slim
RUN apt-get update && \
        apt-get install -y libssl-dev && \
        rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/n2i-power/target/release/n2i-power .

# executable
ENTRYPOINT [ "./n2i-power" ]

# Build
# $ docker build . -t n2i-power:latest

# Run
# $ docker run --restart=always -d --name n2i-power n2i-power:latest
