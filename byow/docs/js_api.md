# Javascript APIs and How to Call Them

Here's a quick list of APIs provided from `bindings.rs`:
 - `log!(format, args...)`
 - `register_image(path, image_id)`
 - `ctx.draw_image(image_id, x, y, width, height)`
 - `ctx.set_fill_color(color)`
 - `ctx.fill_rect(x, y, width, height)`
 - `ctx.set_font(font)`
 - `ctx.fill_text(text, x, y)`
 - `storage.set_item(key, value)`
 - `storage.get_item(key)`
 - `storage.remove_item(key)`
 - `storage.clear()`

Most of these methods are tied to a `&RenderingContext` or a `&Storage`. The type `RenderingContext` is a binding to [CanvasRenderingContext2D](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D), which is a JS object for drawing things on the canvas. The type `Storage` is a binding to [Window.localStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage), which is like a hashmap that persists across webpage loads. 

## The `log!` macro

This macro works exactly like `println!()`, but will print to the javascript console. 

Example usages:
```rust
fn example() {
    let v = vec![1, 2, 3];
    log!("v is {:?}", v);
    log!("{} {} {}", "I", "like", "cheese");
    log!("it is {:.2} degrees today", 451.00123);
}
```

## `register_image(path, image_id)`

This function "registers" an image with the JS side, allowing us to call `ctx.draw_image` later. Some time needs to pass between calling `register_image` and calling `ctx.draw_image`, to allow the image to load.

Usually, the only place this needs to be called is `Binding::new`, to register all the images as soon as the page is loaded.

Example usages:
```rust
fn run_first() {
    register_image("cheese.png", 42);
    register_image("potato.png", 123456789);
}

fn run_later(ctx: &RenderingContext) {
    // will draw cheese.png, if loaded
    // image will occupy a 10px by 10px square on the canvas
    ctx.draw_image(42, 0., 0., 10., 10.);
    
    // will draw potato.png, if loaded
    // image will occupy a 10px by 10px square on the canvas
    ctx.draw_image(123456789, 10., 0., 10., 10.);
}
```
You might be wondering why we use numeric IDs, rather than just using the filename as the ID. After all, wouldn't it be more readable to call `ctx.draw_image("cheese.png", ...)` instead of `ctx.draw_image(42, ...)`?

Unfortunately, passing strings between JS and Rust is really, really slow. Rust uses UTF-8 strings, and JS uses UTF-16 strings - which means that if we used strings as IDs, the JS "glue" code would spend tons of time decoding UTF-8 into UTF-16. Since some games need to draw images a lot (100,000s of images per second), using strings as IDs would contribute significantly to global warming.

## `ctx.draw_image(image_id, x, y, width, height)`

Draws an image on the canvas. The `image_id` must be registered with `register_image`. If the image hasn't loaded yet, nothing will be drawn.

The parameters `x` and `y` refer to where the upper-left corner of the image will be drawn. The drawn image will be resized to `width` by `height`, so the original size of the image doesn't really matter.

Example usages: see `register_image` example.

## `ctx.set_fill_color(color)`

Sets the fill color of the canvas, which future `fill_rect` and `fill_text` calls will use. The parameter `color` is a `u32` in rgba format.

Example usages:
```rust
fn example(ctx: &RenderingContext) {
    ctx.set_fill_color(0xffafafff); // pink, fully opaque
    ctx.fill_text("cheese", 25., 5.); // pink text
    ctx.fill_rect(0., 20., 50., 30.); // pink rectangle
    
    ctx.set_fill_color(BLUE); // from colors.rs
    ctx.fill_text("cheese", 75., 5.); // blue text
    ctx.fill_rect(50., 20., 50., 30.); // blue rectangle
}
```

## `ctx.fill_rect(x, y, width, height)`

Fills in a rectangle with the canvas's fill color. The parameters `x` and `y` specify where the top-left corner of the rectangle will be drawn, and `width` and `height` specify the size of the rectangle.

Example usages: see `set_fill_color` example.

## `ctx.set_font(font)`

Sets the font and font size for drawing text.

Example usages:
```rust
fn example(ctx: &RenderingContext) {
    let line_height = 10;

    // blue text, 10px high, centered at (25, 5)
    ctx.set_font(&format!("{}px ui-monospace", line_height));
    ctx.set_fill_color(BLUE);
    ctx.fill_text("cheese", 25., 5.);
    
    // green text, 20px high, centered at (50, 10)
    ctx.set_font(&format!("{}px Arial", line_height * 2));
    ctx.set_fill_color(GREEN);
    ctx.fill_text("potato", 50., 10.);
}
```

## `ctx.fill_text(text, x, y)`

Draws some text with the canvas's fill color and the canvas's font. `x` and `y` refer to the _center_ of the text, NOT the upper-left corner.

Example usages: see `set_font` example.

## `storage.set_item(key, value)`

Puts a key-value pair in the storage object. The storage object (and anything you put in it) will persist even if the page is reloaded.

Example usages:
```rust
fn run_first(storage: &Storage) {
    storage.set_item("song", "body once told me");
    let s1 = storage.get_item("song");
    let s2 = storage.get_item("dummy");
    // s1 is Some("body once told me")
    // s2 is None
}

fn run_later(storage: &Storage) {
    match storage.get_item("song") {
        Some(mut s) => {
            s.push_str(" the world was gonna roll me");
            storage.set_item("song", &s);
        }
        None => {} // song was not set :(
    }
}
```

## `storage.get_item(key)`

Gets a value from the storage object, returning None if the key is not in storage.

Example usages: see `set_item` example.

## `storage.remove_item(key)`

Removes a key-value pair from the storage object if it exists.

Example usages:
```rust
fn example(storage: &Storage) {
    storage.set_item("cheese", "I like cheese");
    storage.set_item("potayto", "potahto");
    storage.remove_item("cheese");
    let s1 = storage.get_item("cheese");
    let s2 = storage.get_item("potayto");
    // s1 is None
    // s2 is Some("potahto")
}
```

## `storage.clear()`

Removes all key-value pairs from the storage object.

Example usages:
```rust
fn example(storage: &Storage) {
    storage.set_item("cheese", "I like cheese");
    storage.set_item("potayto", "potahto");
    storage.clear();
    let s1 = storage.get_item("cheese");
    let s2 = storage.get_item("potayto");
    // s1 is None
    // s2 is None
}
```