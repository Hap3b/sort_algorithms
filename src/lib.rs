pub mod bogo_sort;
pub mod bubble_sort;
pub mod cocktail_shaker;

fn is_sorted<T: PartialOrd>(tab: &[T], len: usize) -> bool {
    for pos in 0..(len - 1) {
        if tab[pos] > tab[pos + 1] {
            return false;
        };
    };
    true
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
}
