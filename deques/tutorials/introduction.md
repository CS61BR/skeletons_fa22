# Starter Files

When you open up the `deques/` module, you should be greeted with the following:

```
deques/
 - tests/
   - linkedlistdeque.rs
   - mod.rs
 - mod.rs
```
The top level `mod.rs` will contain the `Deque` and `Average` traits, as well as `mod` statements for the overall module structure. Throughout the course of this project, you should be adding files to the structure, until the project structure looks roughly like this:

```
deques/
 - tests/
   - arraydeque.rs
   - averagingdeque.rs
   - linkedlistdeque.rs
   - mod.rs
 - arraydeque.rs
 - averagingdeque.rs
 - linkedlistdeque.rs
 - mod.rs
```

You may have more files, but please don't try to have fewer than this.


# The Deque trait

If you open up `mod.rs`, you should see the following trait definition:
```
pub trait Deque: Display {
    type Item;
    fn new() -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    ...more definitions...
    fn get_last_mut(&mut self) -> Option<&mut Self::Item>;
}

```

Breaking this down,
 - `pub` means the trait can be used outside of the `deques/` module
 - `trait Deque {}` defines a trait named `Deque`
 - `: Display` is a _trait bound_. This means that everything that implements `Deque` must implement `Display` first. In other words, `Deque` is a superset of `Display`.
 - `type Item` is an _associated type_. Essentially, everything that implements `Deque` must specify what type their Deque item is. It's very similar to a generic parameter (we could have defined `Deque` as `pub trait Deque<Item>`), except if we made `Item` a generic parameter, a type may try to implement `Deque` multiple times with different `Item`s.
 - All traits have a default associated type, `Self`. The signature `fn new() -> Self` is essentially a constructor.
 - `&self` is shorhand for `self: &Self`
 - `is_empty` is a _default implementation_. Any type implementing `Deque` may override it, or just leave it.
 - `Option<&mut Self::Item>` is an `Option`, which may have a mutable reference to an item. In other words:
     - If the deque is empty, `get_last_mut` will return `None`
     - Otherwise, it'll return `Some(&mut Item)`, which is a mutable reference to an item. You can use this to assign to the last element, for example:
        ```
        let mut my_deque = ArrayDeque::<i32>::new();
        if let Some(r) = my_deque.get_last_mut() {
            *r = 42;
        }
        ```
