extern crate creusot_contracts;
use creusot_contracts::*;

// Implement and prove the Euclidean division

#[requires(@b != 0)]
#[requires(@a >= 0 && @b >= 0)]
#[requires(@a >= @b)]
#[ensures(exists<r: Int> @a == (@b * @result) + r && 0 <= r && r < @b)]
#[ensures(@result >= 0)]
fn divide(a: i64, b: i64) -> i64 {
    let mut r = a;
    let mut q = 0;

    #[invariant(reverse, a == (b * q) + r)]
    #[invariant(q_bounds, @q >= 0 && q <= a)]
    #[invariant(r_bounds, @r >= 0 && q <= a)]
    while r >= b {
        r -= b;
        q += 1;
    }

    q
}
