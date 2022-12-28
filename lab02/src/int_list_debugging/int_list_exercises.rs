use super::{int_list::IntList, primes::is_prime};

// Part A: (Buggy) mutative method that adds a constant C to each element
pub fn add_constant(lst: &mut IntList, c: i32) {
    let mut head = lst;
    loop {
        if let IntList::More(v, next) = head {
            if let IntList::Empty = **next {
                break;
            } else {
                head = next;
            }
            *v += c;
        }
    }
}

// Part B: (Buggy) mutative method that sets each node value to 0 if the max
// value in the list after the node (inclusive of the node) has the same first
// and last digit
// i.e. [111 -> 67 -> 23 -> 44 -> 13] should turn into [0 -> 67 -> 0 -> 0 -> 13]
pub fn set_to_zero_if_max_fel(lst: &mut IntList) {
    let mut head = lst;
    while let IntList::More(v, next) = head {
        if first_digit_equals_last_digit(max(*v, next)) {
            *v = 0;
        }
        head = next;
    }
}

// Computes the max of a list + an additional number
// why the additional number? Because an empty list doesn't have a
// maximum, so this guarantees a max will always exist
fn max(num: i32, lst: &IntList) -> i32 {
    let mut max = num;
    let mut head = lst;
    while let IntList::More(v, next) = head {
        if *v > max {
            max = *v;
        }
        head = next;
    }
    max
}

fn first_digit_equals_last_digit(mut x: i32) -> bool {
    let last = x % 10;
    while x > 10 {
        x /= 10;
    }
    let first = x % 10;
    first == last
}

// Part C: (Buggy) mutative method that squares each prime element, returning
// whether there were any primes in the list
pub fn square_primes(lst: &mut IntList) -> bool {
    match lst {
        IntList::Empty => false,
        IntList::More(v, next) => {
            let cur_is_prime = is_prime(*v);
            if cur_is_prime {
                *v *= *v;
            }
            cur_is_prime || square_primes(next)
        }
    }
}
