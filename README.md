# CryptoTools

Doing Maths in Rust for some Crypto related applications.

## Dev

```bash
# run the tests
cargo test

# compile
rustc src/lib.rs
```

Note that you can prepend these commands with `docker run --rm -it -u 1000:1000 -v $(pwd):/app -w /app scorpil/rust` if you do not have a Rust environment installed.
