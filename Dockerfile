# Use an official Ubuntu image as the base image
FROM ubuntu:latest

# Set the label for the image
LABEL authors="sxy"

# Set the working directory inside the container
WORKDIR /app

# Set the HOME environment variable
ENV HOME=/root

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Install Rust, Cargo, build-essential, llvm, clang, libclang, gcc, pkg-config, OpenCV, and libmysqlclient-dev
RUN apt-get update && \
    apt-get install -y curl build-essential llvm-14 clang-14 libclang-14-dev gcc pkg-config libopencv-dev fontconfig libfontconfig1-dev libmysqlclient-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.80.1 && \
    . $HOME/.cargo/env && \
    cargo fetch

# Set environment variables for llvm-config, libclang, pkg-config, and mysqlclient
ENV LLVM_CONFIG_PATH=/usr/lib/llvm-14/bin/llvm-config
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig
ENV MYSQLCLIENT_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV MYSQLCLIENT_VERSION=8.0

# Copy the source code
COPY src ./src
COPY tmp ./tmp
COPY data ./data

# Build the application
RUN . $HOME/.cargo/env && cargo build --release

# Expose the port the app runs on
EXPOSE 8081

# Start the application
CMD ["./target/release/photosprocess"]