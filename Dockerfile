FROM rust:slim-bookworm

# View app name in Cargo.toml
ARG APP_NAME=rust-calculator

WORKDIR /app

COPY . .
RUN cargo build --release --locked
RUN cp ./target/release/$APP_NAME /bin/server

CMD ["/bin/server"]
