fn main() {
    uniffi::uniffi_bindgen_main()
}
// cargo run --bin uniffi-bindgen generate --library ../ffi/target/release/libimffi.a --language swift --out-dir out
