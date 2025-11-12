FROM nim65s/cargo-binstall AS installer

# Install Build and Runtime Dependencies
RUN cargo binstall -y --locked \
	cargo-watch \
	just




FROM rust:1.91-bookworm AS runner

ENV DEBIAN_FRONTEND=nointeractive

# Setup the user that will run the project.
RUN useradd -m dev
WORKDIR /home/dev/memwbot

RUN mkdir target dist
RUN chown -R dev:dev .

USER dev

# Setup the cargo path and move Dependencies.
ENV PATH="/home/dev/.cargo/bin:$PATH"

COPY --from=installer /usr/local/cargo/bin/cargo-watch /home/dev/.cargo/bin/cargo-watch
COPY --from=installer /usr/local/cargo/bin/just /home/dev/.cargo/bin/just

# Add the target to run the project
RUN rustup target add wasm32-unknown-unknown

# Run the both the backend and frontend as specified in the dev recipe.
CMD ["just", "dev"]
