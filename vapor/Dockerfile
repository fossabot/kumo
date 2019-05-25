FROM rust:1.35

COPY . /app

RUN cd /app/vapor \
  && cargo build --release --bin=vapor \
  && cp ./target/release/vapor /usr/bin/vapor \
  && cd / \
  && rm -rf app

ENTRYPOINT ["/usr/bin/vapor"]
