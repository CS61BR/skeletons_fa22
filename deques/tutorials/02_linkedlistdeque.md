# Requirements

A regular-ol' deque. Must be `pub struct LinkedListDeque<T>` and implement `Deque`. 

# Layout

For our linked list deque, we won't be using an actual doubly-linked list (i.e. with pointers the way we did with `SLList` in lab03). Instead, we'll be using indices of a Vec as our "pointers".

(Why not use `Rc`? Becuase `Rc` is terrible and bad and also terrible. We have an inherently circular structure and it would be very easy to cause memory leaks, even with a "working" doubly-linked-list. I refuse to teach such blasphemy - but if you really want to learn it, go read [A Bad but Safe Doubly-Linked Deque](https://rust-unofficial.github.io/too-many-lists/fourth.html))

Our layout will be:
```
struct Node<T> {
    item: T,
    prev: usize,
    next: usize
}

pub struct LinkedListDeque<T> {
    nodes: Vec<Node<T>>
}
```

# Strategy

In order to keep things as simple as possible, we'll restrict the item type to `Default`, so we can create a sentinel node. The sentinel node will occupy index 0, meaning that the length of `nodes` will always be one larger than the length of the deque.

The sentinel's `next` will point to the first element in the deque, and the sentinel's `prev` will point to the last element in the deque, forming a ring with the sentinel in the middle.

How do we add elements? We can simply push new `Node`s onto the end of `nodes`, and then set up all the pointers correctly. This involves setting the `prev` and `next` of the new node, as well as the `prev` and `next` of the two nodes that will sit around the new node in the ring.

How do remove elements? This is tricky, especially since we can only pop elements from the end of the `nodes` vector. The strategy is as follows:
 - First, we can swap the node we want to remove with the last node in `nodes`. To accomplish this swap without breaking the ring structure, we'll have to also set the `prev` and `next` of the 4 nodes pointing to the swapping nodes. This isn't too bad, but we also have to deal with the following corner cases:
   - the node to remove might already be the last node, in which case we should do nothing
   - the node to remove might be "next to" the last node in the ring. To avoid problems, we should perform all `prev` and `next` reassignments before performing the swap.
 - Next, we can `pop()` the node off of `nodes`, and perform pointer reassignments to fix the ring structure.


# Implementing Display and Deque traits

You'll probably want to start with these blocks:
```
impl<T: Default + Display> Display for LinkedListDeque<T> {
}

impl<T: Default + Display> Deque for LinkedListDeque<T> {
    type Item = T;
}
```

We restrict our `Item` type to `Default` because we want a sentinel node, and we restrict it to `Display` so we can display it.