# builder image
FROM rust:1.48 as builder
WORKDIR /usr/src/n2i-power
COPY . .
RUN cargo install --path .

# generate clean, final image for end users
FROM alpine:latest
COPY --from=builder /usr/src/n2i-power/target/release/n2i-power .

# executable
ENTRYPOINT [ "./n2i-power" ]

# Build
# $ docker build . -t n2i-power:latest

# Run
# $ docker run --restart=always -d --name n2i-power n2i-power:latest
