killall -9 relay-server
export RUST_BACKTRACE=1
cargo run --bin relay-server &
cargo run --bin relay-server-test
