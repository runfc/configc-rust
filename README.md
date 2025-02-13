# Configc-rust - Rust-based configuration management library powered by configc

*DISCLAIMER: Please do not use this in production, it not even close to be ready!*

----

Configc-rust is configuration management library powered by configc library. Basically, it binds the configc and exposes
a rust-based crate, so you can embed configc capabilities into your rust programs.

Check it out the original: https://github.com/runfc/configc

## Hacking

In order to build this project, you have to download the configc library and at place it at the root of this project
under the name config.c

1. Download the configc library

```bash
git clone https://github.com/runfc/configc.git config.c
```

2. Build the project using Cargo

```bash
cargo build
```

Contributions are more than welcome, feel free to open a Merge Request.

## Support

Feel free to open an issue on https://github.com/runfc/configc-rust/issues
