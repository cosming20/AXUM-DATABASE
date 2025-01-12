need to install rust, last version of cargo leptos 0.2.24 was bindgen 0.2.99

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --version 0.2.24
[linux]  sudo apt install build-essential
cargo install diesel_cli --no-default-features --features mysql
diesel setup
diesel migration run
cargo install wasm-bindgen-cli --version 0.2.99

cargo leptos watch

DATABASE_URL=mysql://root:<password>@127.0.0.1:3306/<database_name>