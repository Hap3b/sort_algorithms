use rand::{Rng, thread_rng};

fn random_shake<T: PartialOrd>(tab: &mut [T], len: usize) {
    let mut rng = thread_rng();

    for pos in 0..len {
        let rdm_pos  = rng.gen_range(0..len);
        tab.swap(pos, rdm_pos);
    }
}

fn is_sorted<T: PartialOrd>(tab: &[T], len: usize) -> bool {
    for pos in 0..(len - 1) {
        if tab[pos] > tab[pos + 1] {
            return false;
        };
    };
    true
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
    fn sorted_array() {
        let array = [1, 2, 3, 4];
        assert!(is_sorted(&array, 4));
    }
    
    #[test]
    fn unsorted_array() {
        let array = [4, 8, 6, 3];
        assert!(!is_sorted(&array, 4));
    }

    #[test]
    fn bogo_test() {
        let mut array = [48, 6, 79, 45];
        bogo_sort(&mut array, 4);
        assert!(is_sorted(&array, 4));
    }
}
