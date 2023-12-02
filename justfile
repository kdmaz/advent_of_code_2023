set shell := ["powershell.exe", "-c"]

gen:
    cargo generate --init --path ./template --name empty

run day part:
    cargo run -p {{day}} --bin {{part}}

fmt:
    cargo fmt
    cargo clippy

test day +opts='':
    cargo test -p {{day}} {{opts}}

bench day:
    cargo bench -q -p {{day}} --bench {{day}}_bench