# Differences from the Java version

## The Render Loop

The biggest difference between the Java version and the Rust version of this project is that the Java version is built around _polling_, while the Rust version is _event driven_.

Specifically, many games in the Java version will look something like this (details omitted):

```java
while (!gameOver) {
    if (StdDraw.hasNextKeyTyped()) {
        ... // process input
    }

    double x = StdDraw.mouseX();
    double y = StdDraw.mouseY();
    ... // stuff
    draw_stuff();
}
```
where the game needs to constantly check for input.

In contrast, the Rust skeleton looks like this (details omitted):
```rust
pub fn handle_keydown(&mut self, keycode: &str) {
    ... // process input
}

pub fn handle_mousemove(&mut self, x: f64, y: f64) {
    ... // stuff
}

pub fn draw_animation_frame(&mut self) {
    draw_stuff();
}
```
where instead of the game constantly checking for input in a loop, it waits for things to happen and responds to them. Note that the above example is set up to handle the following events:
 - a key gets pressed -> `handle_keydown` is called
 - the mouse is moved -> `handle_mousemove` is called
 - the screen has refreshed -> `draw_animation_frame` is called


## The Coordinate System

In the Java version, (0, 0) is at the bottom left of the screen, with increasing y going up the screen. In the Java version, the game specifies how large the application window will be.

In the Rust version, (0, 0) is at the top left of the screen, with increasing y going down the screen. In the Rust version, the user decides how large the window will be, and the game is given `window_width` and `window_height`.

## Saving Information

In the Java version, the game is saved to a _save file_ somewhere on the user's computer. In the Rust version, the game will be saved using [Window.localStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage), which is like a hashmap that persists across reloads. If you've heard of browser cookies, localStorage is basically a better version of that.

## Artistic Differences

Some colors and tiles may be slightly different. For example, the grass tile in the Rust version has a dark green background, instead of a black background. 