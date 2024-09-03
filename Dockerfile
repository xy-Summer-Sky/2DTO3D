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

# Install Rust, Cargo, build-essential, llvm, clang, libclang, gcc, pkg-config, and OpenCV
RUN apt-get update && \
    apt-get install -y curl build-essential llvm-14 clang-14 libclang-14-dev gcc pkg-config libopencv-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.80.1 && \
    . $HOME/.cargo/env && \
    cargo fetch

# Set environment variables for llvm-config, libclang, and pkg-config
ENV LLVM_CONFIG_PATH=/usr/lib/llvm-14/bin/llvm-config
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# Copy the source code
COPY src ./src

# Build the application
RUN . $HOME/.cargo/env && cargo build --release

# Expose the port the app runs on
EXPOSE 8080

# Start the application
CMD ["./target/release/photosprocess"]