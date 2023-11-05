#!/bin/bash

cargo build --release

mv ./target/wasm32-wasi/release/zellij-jump-list.wasm ~/.config/zellij/plugins/

