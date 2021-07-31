# Salsa20 in Rust

__Do not use this library for encrypting data in real world projects! This is for educational purposes only!__

This library implements [Salsa20](https://cr.yp.to/salsa20.html) in [Rust](https://rust-lang.org/), which is a stream cipher developed by [Daniel J. Bernstein](https://cr.yp.to/djb.html).


## Development

Please open a GitHub issue for any problems with the library, including security problems.

### Goals

- [x] Implement the entire Salsa20 specification in safe Rust.
- [ ] Support concurrency/multi-threading.
- [ ] Ciphertext seeking mode.


## Test Vectors

Running `cargo test` will execute all unit tests that implement the test vectors, as provided by the [official Salsa20 specification (PDF)](https://cr.yp.to/snuffle/spec.pdf).