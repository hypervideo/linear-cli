default:
    just --list

install:
    nix profile install .#lr

build:
    nix build .#lr

run *args="":
    cargo run -- {{ args }}
