#! /bin/bash

cargo build &&
	cargo ndk -o ./bindings/android/exampleapp/app/src/main/jniLibs --manifest-path ./Cargo.toml -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build --release &&
	cargo run --bin uniffi-bindgen generate --library ./target/debug/libandroid_bindings.so --language kotlin --out-dir ./bindings/android/exampleapp/app/src/main/java/com/example/rust_android
