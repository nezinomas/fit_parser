FROM rust:latest AS builder-aarch64

RUN apt-get update && apt-get upgrade -y
RUN apt install -y \
    g++-aarch64-linux-gnu libc6-dev-arm64-cross \
    python3-dev \
    python3-pip \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add aarch64-unknown-linux-gnu
# RUN rustup toolchain install stable-aarch64-unknown-linux-gnu

# Install maturin
RUN pip3 install maturin --break-system-packages

WORKDIR /app

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

#01 docker build -t rust-manylinux-aarch64 --target builder-aarch64 .
#02 docker run -v d:/web/maps_project/fit_parser:/app -w /app -it rust-manylinux-aarch64 bash
#03 maturin build --release --target aarch64-unknown-linux-gnu -i python3.11



FROM rust:latest AS builder-x86_64

RUN apt-get update && apt-get upgrade -y
RUN apt install -y \
    g++-x86-64-linux-gnu \
    libc6-dev-amd64-cross \
    python3-dev \
    python3-pip \
    python3-venv \
    && rm -rf /var/lib/apt/lists/*

# Install maturin
RUN pip3 install maturin --break-system-packages


RUN rustup target add x86_64-unknown-linux-gnu

WORKDIR /app


#01 docker build -t rust-manylinux-x86_64 --target builder-x86_64 .
#02 docker run -v d:/web/maps_project/fit_parser:/app -w /app -it rust-manylinux-x86_64 bash
#03 maturin build --release --target x86_64-unknown-linux-gnu -i python3.11