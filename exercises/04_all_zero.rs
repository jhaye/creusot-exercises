extern crate creusot_contracts;
use creusot_contracts::*;

// Prove that after the call to this function the vector only contains zeroes
// Also show that no elements were added or removed
#[ensures(forall<i: Int> 0 <= i && i < (@v).len() ==> @(@^v)[i] == 0)]
#[ensures((@*v).len() == (@^v).len())]
pub fn all_zero(v: &mut Vec<u32>) {
    let mut i = 0;
    let old_v = ghost! { v };
    // Until https://gitlab.inria.fr/why3/why3/-/merge_requests/667 is merged
    // the following invariant is needed to allow Why3 to remember prophecies dont change
    #[invariant(proph_const, ^v == ^old_v.inner())]
    #[invariant(zeroed, forall<j: Int> 0 <= j && j < @i ==> @(@v)[j] == 0)]
    #[invariant(no_size_change, (@v).len() == (@(old_v.inner())).len())]
    while i < v.len() {
        v[i] = 0;
        i += 1;
    }
}
