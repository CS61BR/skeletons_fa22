# Requirements

A deque with an average! Must be `pub struct AveragingDeque<T>` and implement `Deque` and `Average`. 

# Inheritance is bad
The goal of the `AveragingDeque` is to extend the functionality of one of our existing deques, and tack on an `average()` function. In an OOP language, we might be tempted to say,
> Wow, what a perfect use case for inheritance! We can simply write 
> ```
> public class AveragingDeque<T> extends ArrayDeque<T> {
>     ...
> }
> ```
> and we'll get almost all the functionality for free! What an elegant solution!

but like most uses of inheritance, this would be a mistake. To start with, we'll be overriding any methods that modify the deque (`add_first`, `remove_last`, etc). Given that this is more than half of our methods, the "free functionality" isn't that appealing anymore. Secondly, we have to be carely to intercept _every_ method that modifies the deque. That may not seem like a big deal right now, but if somebody decided to add a method to `ArrayDeque`,
```
public void addSecond(T item) {...}
```
everything would fall apart.

Luckily, Rust is not an OOP language, so we don't need to bother ourselves with such foolishness.

# Layout

We'll go with a very simple layout:
```
pub struct AveragingDeque<T> {
    base: ArrayDeque<T>,
    sum: f64
}
```

(feel free to replace use a `LinkedListDeque<T>` instead of an `ArrayDeque<T>`; it doesn't matter which one we use) This layout mimics inheritance, but does not share its pitfalls.

# Strategy

We'll be forwarding most of our calls to `base`, but also modifying a few methods. Specifically, `add_first`, `add_last`, `remove_first`, and `remove_last` should all update `sum` accordingly.

One important detail: __`get_first_mut` and `get_last_mut` must always return `None`__. We can't hand out mutable references, because then someone could update the elements and throw `sum` out of wack.

Since this detail is important, I'll repeat it again: __`get_first_mut` and `get_last_mut` must always return `None`__.

Note that `average()` returns an `Option`; it should return `None` when the length is 0. Please don't divide by 0.

# Implementing Traits

You should start with the following blocks:
```
impl<T: Default+Display+Clone+Into<f64>> Display for AveragingDeque<T> {
}

impl<T: Default+Display+Clone+Into<f64>> Deque for AveragingDeque<T> {
    type Item = T;    
}

impl<T: Default+Display+Clone+Into<f64>> Average for AveragingDeque<T> {
}
```
We use `Default+Display` since `base` will require those; we use `Clone+Into<f64>` so we can turn our items into numbers that we can sum. If we wanted to be extra-general, we could have replaced `f64` with `F: Add+Div+From<usize>`, but that's a bit overkill for our purposes.
