curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
~/.cargo/bin/cargo doc --no-deps
cp ./.doc/index.html ./target/doc/
