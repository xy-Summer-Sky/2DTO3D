# Use an official Rust image as the base image
FROM rust:1.77

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Expose the port the app runs on
EXPOSE 8080

# Start the application
CMD ["./target/release/photosprocess"]