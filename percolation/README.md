# Percolation

Project demo: [percolation](https://sberkun.github.io/percolation)

## Background

This version of the project makes use of the `Result` type. You can learn about it [here](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html). In particular, learning about the `?` operator will be very useful.

## Differences from the Java version

This version of the project has some unique design patterns; see [diffs.md](./diffs.md) for more information.

## Writing Code

You will be implementing the following:
 - `impl Percolatable for Percolation` in `percolation.rs`
 - `calculate_stats` in `percolationstats.rs`

## Running Code


It's suggested that you keep 2 terminals open while working on this project.

In the first terminal, run `wasm-pack build --target web`. This will compile your Rust code into WebAssembly.

In the second terminal, run `python3 -m http.server`. This will start a web server. Now visit http://0.0.0.0:8000/, and you should see the demo!

Whenever you make changes to the code, rerun `wasm-pack build --target web`, and **hard-refresh** the browser page.

## Testing Code

There are no tests provided; you will need to create your own.

When running code in the browser, log messages will show up in the javascript console.


## Cleanup and Submission

make sure no errors from `cargo clippy`, and run `cargo fmt`

no submission this is just for fun
