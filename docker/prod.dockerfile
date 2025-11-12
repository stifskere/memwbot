FROM nim65s/cargo-binstall AS installer

# Install build dependencies.
RUN cargo binstall -y --locked \
	just




FROM rust:1.91-slim-bookworm AS builder

# Setup justfile flag.
ARG BUILD_STAGE
ENV BUILD_STAGE=$BUILD_STAGE

# Copy build dependencies.
ENV PATH="/home/prod/.cargo/bin:$PATH"

COPY --from=installer /usr/local/cargo/bin/just /home/prod/.cargo/bin/just


# Setup user to match running user.
RUN useradd -m prod
RUN chmod 1777 /tmp

# Move into application directory.
WORKDIR /home/prod/app
RUN chown -R prod:prod /home/prod

USER prod

# Setup user environment variables
ENV HOME=/home/prod
ENV CARGO_HOME=/home/prod/.cargo
ENV RUSTUP_HOME=/home/prod/.rustup

# Copy project into application directory.
COPY --chown=prod:prod . .

# Add target and build project.
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown
RUN just build




FROM debian:bookworm-slim AS runner

# Copy built application.
COPY --chown=prod:prod --from=builder /home/prod/app/target/release /home/prod/app/

# Setup running user.
RUN useradd -m prod

# Move into application directory.
WORKDIR /home/prod/app
RUN chown -R prod:prod /home/prod

USER prod

# Setup user environment variables
ENV HOME=/home/prod

# Expose and run built application.
EXPOSE 8080
CMD [ "./memwbot" ]
