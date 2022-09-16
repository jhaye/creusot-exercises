extern crate creusot_contracts;
use creusot_contracts::*;

// Prove that this function implements the sum of the first n natural numbers.
// Hint: there exists a closed form of this sum
#[requires(@n <= 92680)]
#[ensures(@result == ((@n * (@n + 1)) / 2))]
pub fn sum_first_n(n: u32) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    #[invariant(bound, i <= n)]
    #[invariant(sum_step, @sum == ((@i * (@i + 1)) / 2))]
    while i < n {
        i += 1;
        sum += i;
    }
    sum
}
