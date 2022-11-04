---
parent: "Lab 02: Debugging"
title: Summary and Deliverables
nav_order: 2
---


## Writing Code

You will be testing the following methods:
 - `rounded_division` (fix optional)
 - `sum_of_elementwise_maxes` and helpers (fix required)
 - `product` and `sum` (fix required)
 - `add_constant` (fix required)
 - `set_to_zero_if_max_fel` and helpers (fix required)
 - `square_primes` and helpers (fix required)

The given tests are not comprehensive; you are encouraged to write additional tests.

## Running Code

For the first two exercises:
 - `cargo run --bin debug_exercise_1`
 - `cargo run --bin debug_exercise_2`

See [Background](background.md) for how to debug them.

## Testing Code

You will be using both `cargo test` and `rust-gdb`. See [Background](background.md) for how to debug them.

## Submitting Code

 - run `cargo clippy`, and fix any warnings that appear
 - run `cargo fmt`
 - commit and push your code
 - submit to the Gradescope assignment
