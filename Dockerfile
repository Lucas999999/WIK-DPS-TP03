FROM rust:latest
WORKDIR /usr/src/TP-WIK-DPS-03
COPY . .
RUN cargo build
CMD ["./target/debug/TP-WIK-DPS-TP03"]