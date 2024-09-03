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

# Install Rust, Cargo, build-essential, llvm, and clang
RUN apt-get update && \
    apt-get install -y curl build-essential llvm clang && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . $HOME/.cargo/env && \
    cargo fetch

# Set environment variables for llvm-config and libclang
ENV LLVM_CONFIG_PATH=/usr/bin/llvm-config
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib

# Copy the source code
COPY src ./src

# Build the application
RUN . $HOME/.cargo/env && cargo build --release

# Expose the port the app runs on
EXPOSE 8080

# Start the application
CMD ["./target/release/photosprocess"]