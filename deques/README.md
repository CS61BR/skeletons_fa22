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

## Writing Code

For the first part of the project, you will be creating the following `Deque` implementations:
 - `LinkedListDeque`
 - `ArrayDeque`
 - `AveragingDeque`

For the second part of the project, you will be implementing the following methods in `GuitarString`:
 - coming soon! TODO

## Running Code

It's suggested that you keep 2 terminals open while working on this project.

In the first terminal, run `wasm-pack build --target web`. This will compile your Rust code into WebAssembly.

In the second terminal, run `python3 -m http.server`. This will start a web server. Now visit http://0.0.0.0:8000/, and you should see the game!


Whenever you make changes to the code, rerun `wasm-pack build --target web`, and **hard-refresh** the browser page.

## Testing Code

 - Deques: Simple tests for `LinkedListDeque` are provided; you should add your own tests to the `tests/` folder.
 - GuitarString: coming soon! TODO

You can run them with `cargo test`

## Cleanup and Submission

make sure no errors from `cargo clippy`, and run `cargo fmt`

no submission this is just for fun
