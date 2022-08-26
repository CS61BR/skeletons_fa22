# 61b_rust_skeletons

If you have way too much time on your hands, you can now complete 61b assignments in Rust! Because Rust is good and Java is bad.

This is not official course material and is not affiliated with 61b in any way.

## How to complete these assignments

There is no spec. You will have to guess what the spec would be if one existed.

There is no autograder. The real autograder is the friends we made along the way.

Good luck!

## Helpful tips

To get up and running with Rust, you will need:
 - `rustup`: install the Rust toolchain from https://rustup.rs/. This should install `cargo` for you, which contains all the tools needed to work on Rust code.
 - `wasm-pack`: Some of these assignments use WebAssembly and run in the browser, so to compile them, you will need `wasm-pack` from here: https://rustwasm.github.io/wasm-pack/.
 - VSCode with the rust-analyzer extension, or Neovim with the rust-analyzer language server. Intellij also has a Rust extension, but I prefer rust-analyzer.

To learn Rust, check out https://www.rust-lang.org/learn. At minimum, you should go through the first few chapters of [the book](https://doc.rust-lang.org/book/title-page.html) - Rust has a relatively steep learning curve in the beginning, and several unique concepts not found in other languages.

Once you've done the above, you're ready to complete these assignments! There are READMEs in each assignment folder.

Because you are a civilized person, you should always run `cargo clippy` and `cargo fmt` before committing your code. Only a barbarian would ignore code style errors, or worse, try to fix them by hand.
