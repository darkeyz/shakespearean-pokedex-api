FROM rustlang/rust:nightly

RUN rustup default nightly && rustup update

RUN cargo install cargo-watch

WORKDIR /usr/src/app

COPY Cargo* Rocket.toml ./
COPY src ./src

EXPOSE 3000

CMD cargo watch -x run
