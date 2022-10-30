# What do the files do?

In this project, you'll be interacting with a large number of source files. This doc explains them.



## `bindings.rs`

Everything we do eventually needs to call Javascript APIs; This file (along with `index.html`) bridges the gap between Rust-land and JS-land.

The first half of `bindings.rs` is primarily to let Rust call JS functions; the seconds half of `bindings.rs` is primarily to let JS call Rust functions (via methods of the `Binding` object). The `Binding` methods should primarily delegate to other functions that do the "actual" work; try to put as little as possible in `bindings.rs`.

### Should you edit this file?

✅ Yes! If you want to handle more input than just keydown, you should edit the methods `handle_keyup`, `handle_mousedown`, etc. These should ideally just delegate to methods of `Game`.

If you want access to more powerful JS APIs (like Datetime, sound, more canvas methods, etc), you can add them to the `extern "C"` block, as well as any "glue" in `index.html` if needed. Refer to the [wasm-bindgen guide](https://rustwasm.github.io/wasm-bindgen/examples/import-js.html) if you get stuck.

## `index.html`

This is a webpage that just contains one full-screen canvas. It also contains some "glue" code needed to call JS APIs from Rust.

### Should you edit this file?

❌ You can if you want to, but it usually won't be necessary. The main reason to edit this file is if you want acess to more powerful JS APIs, as described above (under `bindings.rs`).


## `game.rs`

This contains some starter code for your game.

### Should you edit this file?

✅ Yes! In fact, you should turn this file into an entire folder, so you can have seperate files for world generation, mechanics, input, etc.


## `drawing/animation.rs`

This contains an all-important `draw` function, as well as a few helper functions that you can use. Most of the visuals for your game should be implemented here.

### Should you edit this file?
✅ Yes! In fact, you may want to add additional files to store helper functions. Also, feel free to modify or outright delete the provided helper functions; they're just there to get you started.

## `drawing/tiles.rs`

This contains the `Tile` type definition, some helper functions to create tiles, and a bunch of tiles that you can use in your game.

### Should you edit this file?
✅ Yes! You should add new tiles here, and modify the existing tiles to fit your game (for example, you can change some of the text tiles into image tiles).

## `drawing/images.rs`

This just contains an array of image filenames and their corresponding IDs. All the images in this array will get registered in `Binding::new`.

### Should you edit this file?
✅ Yes! If you add images to your game, you should add them to this array.

## `drawing/colors.rs`

This contains the `Color` type definition, some helper functions to create colors, and a bunch of colors you can use in your game.

### Should you edit this file?
✅ Yes! You should add new colors here.

## `drawing/board.rs`

This contains the `Board` and `DrawingBoard` type definitions, and a helper function for creating `DrawingBoard`s. A `Board` is basically a grid of `Tile`s, and a `DrawingBoard` is just a grid of tiles and a location+size to draw them at.

### Should you edit this file?
❌ You can if you want to, but it usually won't be necessary. If you want to add helper functions here, or change how `Board`/`DrawingBoard` work, feel free.




## `random.rs`

This contains code for psuedo-random number generation.

### Should you edit this file?

❌ You can if you want to, but it usually won't be necessary. For example, if you want random numbers in a Poisson distribution, you can implement an algorithm for that here.

