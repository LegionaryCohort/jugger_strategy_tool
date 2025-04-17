cd bevy
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./wasm-bindgen-out \
    --out-name "jugger_strategy_tool" \
    ./target/wasm32-unknown-unknown/release/bevy.wasm
cd ..
trunk serve