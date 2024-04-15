# Start with a Rust base image to build our application
FROM rust:latest as builder

# Create a new binary project
WORKDIR /usr/src/rust-t5-endpoint

# Copy the source code into the container
COPY ./src ./src
COPY Cargo.toml Cargo.lock ./

# Build the application in release mode
RUN cargo build --release

# Now, start with a fresh image to keep it clean and small
FROM public.ecr.aws/lambda/provided:al2

# Copy the compiled binary from the builder image to the runtime image
COPY --from=builder /usr/src/rust-t5-endpoint/target/release/rust-t5-endpoint /var/task/

# Set the handler executable (which is the binary name by default)
CMD ["rust-t5-endpoint"]
