# FROM docker/whalesay:latest
FROM rust:latest as builder
WORKDIR /usr/src/whynote-backend
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /usr/src/whynote-backend
# RUN apt-get update
# RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/whynote-backend/target/release/backend-axum /usr/local/bin
COPY --from=builder /usr/src/whynote-backend/.env .
EXPOSE 8009
CMD ["backend-axum"]