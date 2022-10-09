RUST_BACKTRACE=1 cargo run --manifest-path ../../spallcomp/Cargo.toml -- "$@"
RUST_BACKTRACE=1 cargo run --manifest-path ../../spallserve/Cargo.toml -- build/ -p 8000