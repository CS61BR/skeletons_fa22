# Requirements

A regular-ol' deque. Must be `pub struct ArrayDeque<T>` and implement `Deque`. 

# Layout

For our array-based (well, slice-based) deque, we'll be using a similar layout as the `AList` from lab03:
```
pub struct ArrayDeque<T> {
    items: Box<[T]>,
    head: usize, // first element - 1
    tail: usize, // last element + 1
}
```

# Strategy

The `Box<[T]>` can be thought of as a fixed-length array: when we need to, we can resize it up or down. We can mimic a circular structure with it: the element before index 0 has the index `items.len() - 1`.



# Helpful code

Similar to the AList, we'll use the following helper function to allocate our `Box<[T]>`s:
```
/* Allocates a fixed-length Box<[T]> */
fn allocate_slice<T: Default>(n: usize) -> Box<[T]> {
    unsafe {
        let layout = std::alloc::Layout::array::<T>(n).unwrap();
        let ptr = std::alloc::alloc(layout) as *mut T;
        for i in 0..n {
            std::ptr::write(ptr.add(i), T::default());
        }
        let slice = core::slice::from_raw_parts_mut(ptr, n);
        Box::from_raw(slice)
    }
}
```
And to implement the `Display` and `Deque` traits, we'll start with the following code blocks:
```
impl<T: Default + Display> Display for ArrayDeque<T> {
}

impl<T: Default + Display> Deque for ArrayDeque<T> {
    type Item = T;
}
```

We restrict our `Item` type to `Default` because we need something to fill our empty spots with, and we restrict it to `Display` so we can display it.
