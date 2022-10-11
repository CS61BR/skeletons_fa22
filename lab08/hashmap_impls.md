# Hashmap implementations

There are many different ways to implement a hashmap; here are a few of the key types:
 - external chaining
     - bucketed: [MyHashMap](#myhashmap)
     - linked lists: [CHashMap](#chashmap)
 - open addressing
     - linear probing: [OHashMap](#ohashmap)
     - quadratic probing ([wikipedia](https://en.wikipedia.org/wiki/Quadratic_probing))
     - double hashing ([wikipedia](https://en.wikipedia.org/wiki/Double_hashing))

There are other design choices, such as how to handle removals in open-addressing schemes. Two common ways are:
 - tombstones: when removing an element from the hashmap, leave a "tombstone" behind to mark that an element was removed. This is what [OHashMap](#ohashmap) does.
 - backshifting: when removing an element from the hashmap, shift all elements with the same hash back one space.

## MyHashMap

MyHashMap is an external chaining implementation with `Vec` buckets.

Pros: This implementation is relatively simple, and is very robust to collisions. This means that if memory footprint is a concern, it can tolerate very high loading factors (such as 100.0 instead of 0.75).

Cons: Each `Vec` bucket is allocated seperately, which means that resizing takes much longer than for the other implementations. This also means that it takes a lot of extra memory at low loading factors. 


## CHashMap

CHashMap is an external chaining implementation with linked lists. The C stands for "cheese".

Pros: This implementation is robust to collisions, although not as robust as `MyHashMap`. However, it tends to have a lower overall memory footprint than `MyHashMap`, and tends to be a bit speedier. This implementation can also resize very quickly, since it doesn't need to allocate new `Vec`s.

Cons: For very high amounts of collisions, linked lists give poor cache performance, making `MyHashMap` the better choice. 


## OHashMap

OHashMap is an open addressing implementation with linear probing and tombstones. The O stands for "open addressing".

Pros: Resizing tends to be slow, but non-resizing operations tend to be faster than other implementations. Note that this particular implementation forgoes some optimizations for simplicity; In contrast, the standard library uses a highly-optimized, "industrial-strength" open-addressing hashmap.

Cons: This implementation is very sensitive to collisions, so a low loading factor must be used. 