!/bin/bash

cargo build --release

mv ./target/wasm32-wasi/debug/zellij-jump-list.wasm ~/.config/zellij/plugins/

