set shell := ["powershell.exe", "-c"]

gen:
    cargo generate --init --path ./template --name empty

run day part:
    cargo run -p {{day}} --bin part{{part}}

fmt:
    cargo fmt
    cargo clippy

test day +opts='':
    cargo test -p {{day}} {{opts}}
