# Deques

## Background

Since the setup for Rust is very different from the Java version, the following tutorials are recommended:
 - introduction
 - linkedlistdeque
 - arraydeque
 - averagingdeque

You can find them in the `tutorials/` folder.

## Differences from the Java version

 - The dequeue interface is different: `get(index)` has been replaced with `get_first` and `get_last` functions
 - The `Deque` trait is provided as starter code
 - Implementing `AveragingDeque` instead of `MaxArrayDeque`
 - Implementing `IntoIter` and `Eq` are not part of the project, although you are certainly welcome to implement them if you want to extra challenge.
 - `tic()` and `sample()` have been combined into `advance()`.

## Writing Code

For the first part of the project, you will be creating the following `Deque` implementations:
 - `LinkedListDeque`
 - `ArrayDeque`
 - `AveragingDeque`

For the second part of the project, you will be implementing the following methods in `guitarstring.rs`:
 - `new`
 - `pluck`
 - `advance`

## Running Code

It's suggested that you keep 2 terminals open while working on this project.

In the first terminal, run `wasm-pack build --target web`. This will compile your Rust code into WebAssembly.

In the second terminal, run `python3 server.py`. This will start a web server. Now visit http://localhost:8000/, and you should see the game! Note:
 - `server.py` sets some security headers so that audio will work
 - similarly, we need to visit `localhost` instead of `0.0.0.0` because `0.0.0.0` is insecure.


Whenever you make changes to the code, rerun `wasm-pack build --target web`, and **hard-refresh** the browser page.

Unfortunely, this project only works in Chrome; various issues keep it from working in any other browser.

## Testing Code

 - Deques: Simple tests for `LinkedListDeque` are provided; you should add your own tests to the `tests/` folder.
 - GuitarString: tests are provided in `guitarstringtests.rs`.

You can run them with `cargo test`.

## Cleanup and Submission

make sure no errors from `cargo clippy`, and run `cargo fmt`

no submission this is just for fun
