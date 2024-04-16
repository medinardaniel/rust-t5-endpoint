# Use the official Rust image for the build stage
FROM rust:latest as builder

# Set an environment variable for the application name
ENV APP rust_lambda_t5

# Set the working directory in the builder stage
WORKDIR /usr/src/$APP

# Copy the entire project to the working directory in the builder stage
COPY . .

# Build the application
RUN cargo build --release

# Output the contents of the release directory to verify the build
RUN ls -la /usr/src/$APP/target/release/

# Start a new stage from scratch to minimize the final image size
FROM public.ecr.aws/lambda/provided:al2 as runtime

# Copy the built executable from the builder stage to the Lambda runtime directory
COPY --from=builder /usr/src/$APP/target/release/rust_lambda_t5 /var/task/bootstrap

# Set the CMD to your handler (bootstrap file in Lambda custom runtime)
CMD ["./bootstrap"]
