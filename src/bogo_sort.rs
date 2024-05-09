use rand::{Rng, thread_rng};
use crate::is_sorted;

fn random_shake<T: PartialOrd>(tab: &mut [T], len: usize) {
    let mut rng = thread_rng();

    for pos in 0..len {
        let rdm_pos  = rng.gen_range(0..len);
        tab.swap(pos, rdm_pos);
    }
}

fn bogo_sort<T: PartialOrd>(tab: &mut [T], len: usize) {
    while !is_sorted(tab, len) {
        random_shake(tab, len);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bogo_test() {
        let mut array = [48, 6, 79, 45];
        bogo_sort(&mut array, 4);
        assert!(is_sorted(&array, 4));
    }
}
