default:
    just --list

install:
    nix profile install .#lr

uninstall:
    nix profile remove packages.x86_64-linux.lr

build:
    nix build .#lr

run *args="":
    cargo run -- {{ args }}
