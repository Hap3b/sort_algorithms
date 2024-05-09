use crate::is_sorted;

fn bubble_sort<T: PartialOrd>(tab: &mut [T], len: usize) {
    let mut is_not_sorted = true;
    while is_not_sorted {
        is_not_sorted = false;
        for i in 1..len {
            if tab[i] < tab[i-1] {
                is_not_sorted = true;
                tab.swap(i-1, i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut array = [5, 4, 3, 2];
        bubble_sort(&mut array, 4);
        assert!(is_sorted(&array, 4));
    }
}
