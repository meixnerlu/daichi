FROM rust:1.83-bullseye as builder
WORKDIR /usr/src/daichi
COPY . .
RUN apt-get update && apt-get install -y libopus-dev
RUN cargo install --path daichi-core

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libopus-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/daichi-core /usr/local/bin/daichi-core
CMD ["daichi-core"]
