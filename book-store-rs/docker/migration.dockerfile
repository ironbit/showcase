FROM rust:slim

RUN apt-get update; \
    apt-get install -y pkg-config libssl-dev rsync; \
    rm -rf /var/lib/apt/lists/*;

RUN cargo install sea-orm-cli; 
