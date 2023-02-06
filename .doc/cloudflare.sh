# install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# generate documentation
~/.cargo/bin/cargo doc --no-deps
# create (copy) an `index.html` because `cargo doc` doesn't create one for workspaces.
cp ./.doc/index.html ./target/doc/
