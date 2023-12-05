# syntax=docker/dockerfile:1.4


FROM rust:bookworm as build-environment


ENV DEBIAN_FRONTEND=noninteractive


WORKDIR /usr/src/etop

COPY . .


RUN cargo build --release --locked \
    && mkdir -p /output \
    && mv target/release/etop /output/  # Updated binary name here


FROM debian:bookworm-slim


ENV DEBIAN_FRONTEND=noninteractive


RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*


COPY --from=build-environment /output/etop /usr/local/bin/  


RUN useradd -ms /bin/bash etopuser
USER etopuser


ENTRYPOINT ["etop"]  
