# GitKV

Disclaimer: this is just a proof of concept I put together to learn Rust.

A simple key value store backed up in git.


## How to use

You'll have to `cargo build` the thing.

I'm using the [git2rs](https://github.com/rust-lang/git2-rs) crate to interact with git.

git2rs uses the openssl crate, and you may need to install some extra
dependencies manually on your system to get it building. See
[openssl](https://github.com/sfackler/rust-openssl) docs for more information.

Once this builds, you can run the app with:
```
$ STORE=/path/to/your/store cargo run -- --help

```
