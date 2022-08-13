# 61b_rust_skeletons

If you have way too much time on your hands, you can now complete 61b assignments in Rust! Because Rust is good and Java is bad.

This is not official course material and is not affiliated with 61b in any way.

## How to complete these assignments

There is no spec. You will have to guess what the spec would be if one existed.

There is no autograder. The real autograder is the friends we made along the way.

Good luck!

## Helpful tips

Most assignments follow the pattern of "fill in the code and run the tests". In this case, you can run the tests using the `cargo test` command.

Some assignments will produce an executable (for example, lab1). You can run the executable via `cargo run`. For example, lab1 can be run with
```
cargo run 2000 2001 2002 2003 2004
```
The other way to run the executable is to build it via `cargo build` (or `cargo build --release`), and run the resulting binary in the `target/` directory.

Because you are a civilized person, you should always run `cargo clippy` and `cargo fmt` before committing your code. Only a barbarian would ignore code style errors, or worse, try to fix them by hand.
