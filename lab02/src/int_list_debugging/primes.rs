use rand::Rng;

/**
 * This (complicated) algorithm returns True if its argument is prime,
 * otherwise False. When you're debugging, stepping into this function may
 * not be the best idea! Consider instead stepping *over* this function,
 * and ensuring that its return value makes sense.
 *
 * If you're curious, this algorithm uses Fermat's Little Theorem as a
 * primality test, and returns the correct answer w.h.p. (due to the presence)
 * of Carmichael numbers. If this makes no sense to you, good! It shouldn't.
 * The goal of this function is to make sure you learn to abstract away the inner
 * workings of a function and debug it as a black-box with the "Step Over" feature.
 *
 * @source: https://www.geeksforgeeks.org/primality-test-set-2-fermet-method/
 * @param n an arbitary integrer
 * @return True iff. the integer is prime
 */
pub fn is_prime(n: i32) -> bool {
    // Corner cases
    if n <= 1 || n == 4 {
        return false;
    }
    if n <= 5 {
        return true;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..3 {
        // try 3 times
        // Pick a random number in [2..n-2)
        // Above corner cases make sure that n > 4
        let a = rng.gen_range(2..n - 2);
        if power_mod(a, n - 1, n) != 1 {
            return false;
        }
    }
    true
}

/**
 * This is a helper method to isPrime. You can ignore this method.
 * It is an iterative Function to calculate a^n mod p in log time
 *
 * @source: https://www.geeksforgeeks.org/primality-test-set-2-fermet-method/
 */
fn power_mod(mut a: i32, mut n: i32, p: i32) -> i32 {
    let mut res = 1;
    a %= p;
    while n > 0 {
        // if n is odd, multiply a with res
        if n % 2 == 1 {
            res = (res * a) % p;
        }
        // n must be even now
        n /= 2;
        a = (a * a) % p;
    }
    res
}
