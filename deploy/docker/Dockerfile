FROM rust:1.78 as build
WORKDIR /app
COPY . .
RUN cargo build -p rivulet-cli --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/rivulet /usr/local/bin/rivulet
CMD ["rivulet", "version"]
