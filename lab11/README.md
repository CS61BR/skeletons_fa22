# Lab 11

## Background

This lab is an intro lab for the BYOW project. As a result, it has nearly the same skeleton. 

Some documentation you should read through:
 - [intro to the skeleton files](../byow/docs/what_do_the_files_do.md)
 - [how to use Javascript APIs](../byow/docs/js_api.md)

## Differences from the Java version

The Rust BYOW skeleton has significant differences from the Java BYOW skeleton; these differences are explained in detail [here](../byow/docs/diffs_from_java.md).  

For lab 11 specifically, the main difference is that the 3 demos are not in seperate files. Instead, they're just functions that can be commented or uncommented in `game.rs`.


## Writing Code

The 4 demos are as follows:
 - boring world: in `game.rs`, in the `render` function, make sure only `boring_world()` is uncommented. Run the code and observe that the world is boring.
 - random world: in `game.rs`, in the `render` function, comment out `boring_world()` and uncomment `random_world()`. Run the code and observe that the world is less boring.
 - plus world: implement code to draw a single plus in `plus_world()`, then edit `render` accordingly. Run the code and observe your beautiful plus.
 - plus world, but tessalated (optional): implement code to draw many tessalated plusses in `plus_world()`. Run the code and observe your beautiful plusses.

## Testing Code

No tests this lab; just make something that looks good!

## Running Code


It's suggested that you keep 2 terminals open while working on this project.

In the first terminal, run `wasm-pack build --target web`. This will compile your Rust code into WebAssembly.

In the second terminal, run `python3 -m http.server`. This will start a web server. Now visit http://0.0.0.0:8000/, and you should see the demo!

Whenever you make changes to the code, rerun `wasm-pack build --target web`, and **hard-refresh** the browser page.

## Cleanup and Submission

make sure no errors from `cargo clippy`, and run `cargo fmt`

no submission this is just for fun