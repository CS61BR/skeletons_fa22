# 2048


## Differences from the Java version

 - the game no longer ends at 2048 (the "max tile"). Just like most other implementations, the game continues until moves can no longer be made, so 4096 and beyond are valid tiles.
 - you are encouraged, but not forced to, make helper methods for `game_over`.
 - not as many tests are needed, since there is no longer a stateful "model" object to test. Also tests relating to the "max tile" have been removed.

## Writing Code

For the Rust version of the project, here's what you need to know:
 - all the code you will be implementing is in `game.rs`. You will be implementing `tilt` and `game_over`
 - it is highly suggested that you use `rotate_board`, `unrotate_board`, and `unrotate_move` in your implementation of `tilt`.


## Running Code

It's suggested that you keep 2 terminals open while working on this project.

In the first terminal, run `wasm-pack build --target web`. This will compile your Rust code into WebAssembly.

In the second terminal, run `python3 -m http.server`. This will start a web server. Now visit http://0.0.0.0:8000/, and you should see the game!


Whenever you make changes to the code, rerun `wasm-pack build --target web`, and **hard-refresh** the browser page.



## Testing Code

`cargo test` for unit tests.

When running code in the browser, log messages will show up in the javascript console.


## Cleanup and Submission

make sure no errors from `cargo clippy`, and run `cargo fmt`

no submission this is just for fun
