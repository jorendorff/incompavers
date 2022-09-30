# Why doesn't this work?

I got errors trying to run Clippy on this project.

Possible steps to reproduce:

1. Have [Homebrew](https://brew.sh/) and [rustup](https://rustup.rs/) installed
2. Run these commands:

    ```
    brew install rust
    git clone https://github.com/jorendorff/incompavers.git
    cd incompavers
    cargo clean && cargo clippy --verbose
    ```

Expected behavior: Clippy should lint the crate.

Actual behavior: Clippy produced the errors below.

```console
❯ rustup show
Default host: x86_64-apple-darwin
rustup home:  /Users/jorendorff/.rustup

installed toolchains
--------------------

stable-x86_64-apple-darwin (default)
nightly-2022-03-27-x86_64-apple-darwin
nightly-x86_64-apple-darwin
stage1
1.58.1-x86_64-apple-darwin
1.59.0-x86_64-apple-darwin
1.60.0-x86_64-apple-darwin
1.61.0-x86_64-apple-darwin
1.62.0-x86_64-apple-darwin
1.63.0-x86_64-apple-darwin
1.64.0-x86_64-apple-darwin

installed targets for active toolchain
--------------------------------------

x86_64-apple-darwin
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-apple-darwin (default)
rustc 1.64.0 (a55dd71d5 2022-09-19)

2022-09-30 14:34:09 ~/src/incompavers (main) ❯ cargo clean && cargo clippy
    Checking base64 v0.13.0
   Compiling paste v1.0.9
    Checking incompavers v0.1.0 (/Users/jorendorff/src/incompavers)
error[E0514]: found crate `paste` compiled by an incompatible version of rustc
 --> src/lib.rs:1:9
  |
1 | pub use paste::paste;
  |         ^^^^^
  |
  = help: please recompile that crate using this compiler (rustc 1.64.0 (a55dd71d5 2022-09-19)) (consider running `cargo clean` first)
  = note: the following crate versions were found:
          crate `paste` compiled by rustc 1.63.0: /Users/jorendorff/src/incompavers/target/debug/deps/libpaste-1cc8d85ad06b00de.dylib

error[E0514]: found crate `base64` compiled by an incompatible version of rustc
  --> src/lib.rs:22:26
   |
22 |     println!("hello {}", base64::encode(b"world"));
   |                          ^^^^^^
   |
   = help: please recompile that crate using this compiler (rustc 1.64.0 (a55dd71d5 2022-09-19)) (consider running `cargo clean` first)
   = note: the following crate versions were found:
           crate `base64` compiled by rustc 1.63.0: /Users/jorendorff/src/incompavers/target/debug/deps/libbase64-bd37c0a895c55f89.rmeta

error: could not compile `incompavers` due to 2 previous errors
```
