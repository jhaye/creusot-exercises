extern crate creusot_contracts;
use creusot_contracts::*;

// Prove the following:
// 1. If we return Some(i) it is the first index containing `tgt`
// 2. If we return None, then there are no indices containing `tgt`

#[ensures(result == None ==>
          forall<i : Int> 0 <= i && i < (@*v).len() ==>
          (@*v)[i] != *tgt)]
#[ensures(exists<i: usize> result == Some(i) ==>
          (@*v)[@i] == *tgt)]
#[ensures(exists<i: usize> result == Some(i) ==>
          forall<j : usize> j < i && (@*v)[@j] != *tgt)]
fn search<T: Ord + Model>(v: &Vec<T>, tgt: &T) -> Option<usize> {
    let mut i: usize = 0;

    #[invariant(bound, @i <= (@*v).len())]
    #[invariant(prev_no_tgt, forall<j : Int> 0 <= j && j < @i ==> (@*v)[j] != *tgt)]
    while i < v.len() {
        if &v[i] == tgt {
            return Some(i);
        }

        i += 1
    }

    return None;
}
