![GitHub](https://img.shields.io/github/license/samuelba/healthcheck)
[![ci](https://github.com/samuelba/healthcheck/actions/workflows/ci.yml/badge.svg)](https://github.com/samuelba/healthcheck/actions/workflows/ci.yml)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/samuelba/healthcheck)
![Docker Image Size (latest semver)](https://img.shields.io/docker/image-size/samuelba/healthcheck)

# Health Check

A minimal health check. Calls the defined API and exits either with 0 (success) or 1 (failure).

## Usage

Example `Dockerfile` file

```Dockerfile
FROM rust:1.68 as builder

# Make use of cache for dependencies.
RUN USER=root cargo new --bin your_app
WORKDIR ./your_app
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release && \
    rm src/*.rs

# Build the app.
COPY . ./
RUN rm ./target/release/deps/your_app*
RUN cargo build --release

# Use distroless as minimal base image to package the app.
FROM gcr.io/distroless/cc-debian11:nonroot

COPY --from=builder --chown=nonroot:nonroot /your_app/target/release/your_app /app/your_app
COPY --from=samuelba/healthcheck:latest --chown=nonroot:nonroot /app/healthcheck /app/healthcheck
USER nonroot
WORKDIR /app
EXPOSE 9000

# Define the port and API path for the healthcheck.
# The health check will call http://localhost:PORT/API_PATH.
ENV PORT=9000
ENV API_PATH=api/v1/health
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s CMD ["/app/healthcheck"]

CMD ["./your_app"]
```
