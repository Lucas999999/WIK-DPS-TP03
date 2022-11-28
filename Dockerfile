FROM rust:latest
WORKDIR /usr/src/TP-WIK-DPS-02
COPY . .
RUN cargo build
CMD ["./target/debug/TP-WIK-DPS-TP02"]