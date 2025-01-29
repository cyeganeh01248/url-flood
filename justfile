default: build


@build:
    cargo build --target-dir ./target

alias build-dev := build

@build-release:
    cargo build --release --target-dir ./target

@test:
    cargo test

@install:
    cargo install --path .

run *ARGS: build-release
    ./target/release/url-flood {{ ARGS }}
