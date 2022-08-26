# Lab 2

## Background

Resources:
 - In addition to IntLists lecture, read this: https://rust-unofficial.github.io/too-many-lists/ 
 - Debugging binaries: https://blog.logrocket.com/debugging-rust-apps-with-gdb/
 - Debugging tests: https://whamcloud.github.io/Online-Help/docs/Contributor_Docs/cd_Debugging_Rust_Tests.html


How to debug a binary:
 - First, run `cargo build`. This will build dev/debug version for all the binaries in the crate.
 - Run `rust-gdb target/debug/[name]`. For example, for the first exercise, you should run `rust-gdb target/debug/debug_exercise_1`. This should bring up a gdb prompt.
 - In the gdb prompt, run `layout src`, and then `refresh` if needed. You should now have a split screen with code on top and a gdb prompt on the bottom.
 - Set a breakpoint where you want to start debugging. For example, I would run `b rounded_division`.
 - In the gdb prompt, run `run` to start the program (optionally with command-line arguments if the program takes them).
 - debug your test! It may help to pull up a GDB cheatsheet.

How to debug a test:

 - First, run `cargo test [name]`. For example, if I wanted to run the `set_to_zero_if_max_fel::test_1` test, I would run `cargo test test_1`. You should see something like 
    ```
        Finished test [unoptimized + debuginfo] target(s) in 0.00s
        Running unittests src/lib.rs (target/debug/deps/lab2-6cc839387471e659)

    running 1 test
    test int_list::tests::set_to_zero_if_max_fel::test_1 ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s
    ```

 - Copy the path provided after "Running unittests ...". You'll now want to run `rust-gdb [path]`. For example, I would run `rust-gdb target/debug/deps/lab2-6cc839387471e659`. This should bring up a gdb prompt.
 - In the gdb prompt, run `layout src`, and then `refresh` if needed. You should now have a split screen with code on top and a gdb prompt on the bottom.
 - Set a breakpoint where you want to start debugging. For example, I would run `b set_to_zero_if_max_fel`.
 - In the gdb prompt, run `run --test [name]`. For example, I would run `run --test test_1`
 - debug your test! It may help to pull up a GDB cheatsheet.
 

 ## Writing Code

You will be testing the following methods:
 - `rounded_division` (fix optional)
 - `sum_of_elementwise_maxes` and helpers (fix required)
 - `product` and `sum`
 - `add_constant` (fix required)
 - `set_to_zero_if_max_fel` and helpers (fix required)
 - `square_primes` and helpers (fix required)

The given tests are not comprehensive; you are encouraged to write additional tests.

## Running Code

For the first two exercises:
 - `cargo run --bin debug_exercise_1`
 - `cargo run --bin debug_exercise_2`

See [Background](#background) above for how to debug them.

## Testing Code

You will be using both `cargo test` and `rust-gdb`. See [Background](#background) above for how to debug them.

## Cleanup and Submission

make sure no errors from `cargo clippy`, and run `cargo fmt`

no submission this is just for fun
