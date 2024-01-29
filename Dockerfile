FROM rust:alpine as build

# create a new empty shell project

RUN apk update && apk add build-base

# Add any additional dependencies your application may need
RUN apk add libgcc libressl-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /koval-telegram-bot

# copy over your manifests
# COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src

# this build step will cache your dependencies
RUN cargo build --target x86_64-unknown-linux-musl --release

# our final base
FROM alpine
# copy the build artifact from the build stage
COPY --from=build /koval-telegram-bot/target/x86_64-unknown-linux-musl/release/koval-telegram-bot ./koval

ENV TELOXIDE_TOKEN=your_token_here
ENV RUST_LOG=info

# set the startup command to run your binary
CMD ["./koval"]
