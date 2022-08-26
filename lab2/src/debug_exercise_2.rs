// Exercise for learning how the step-over feature works.

pub fn main() {
    let a = vec![1, 11, -1, -11];
    let b = vec![3, -3, 2, -1];
    println!("{:?}", sum_of_elementwise_maxes(&a, &b));
}

/* Returns the sum of the element-wise max of a and b.
   For example if a = {2, 0, 10, 14} and b = {-5, 5, 20, 30},
   the result should be 57.

   Notice the rreturn type, Result. Result is used for "fallible" operations:
   it either returns an "Ok" result (here i32) when the oporation succeeds,
   or an Error (here, a String error message) when it fails.

   The "try" (?) operator is used to return Result::Error should it occur.
*/
fn sum_of_elementwise_maxes(a: &Vec<i32>, b: &Vec<i32>) -> Result<i32, String> {
    let maxes = array_max(a, b)?;
    let sum_of_maxes = array_sum(&maxes);
    Ok(sum_of_maxes)
}

fn array_max(a: &Vec<i32>, b: &Vec<i32>) -> Result<Vec<i32>, String> {
    if a.len() != b.len() {
        return Result::Err(String::from("arrays don't have the same length"));
    }
    let mut return_vec = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        let bigger_value = max(a[i], b[i]);
        return_vec.push(bigger_value);
    }
    Ok(return_vec)
}

fn array_sum(v: &Vec<i32>) -> i32 {
    let mut i = 0;
    let mut sum = 0;
    while i < v.len() {
        sum = sum + add(sum, v[i]);
        i += 1;
    }
    sum
}

fn max(a: i32, b: i32) -> i32 {
    // If you're stepping into this function, just step out (gdb command: finish)
    // because you're not going to learn anything
    // If there is a bug here, just rewrite this function from scratch
    let w = (b - a) >> 31;
    let z = !(b - a) >> 31;
    b & w | a & z
}

fn add(a: i32, b: i32) -> i32 {
    // If you're stepping into this function, just step out (gdb command: finish)
    // because you're not going to learn anything
    // If there is a bug here, just rewrite this function from scratch
    let x = a;
    let y = b;
    let mut and = x & y;
    let mut xor = x ^ y;
    while and != 0 {
        and <<= 1;
        let temp = xor ^ and;
        and &= xor;
        xor = temp;
    }
    xor
}
