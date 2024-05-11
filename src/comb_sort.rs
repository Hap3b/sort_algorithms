use crate::is_sorted;

fn comb_sort<T: PartialOrd>(tab: &mut [T], len: usize, k: usize) {
    let mut is_not_sorted = true;
    let mut gap = len;
    while is_not_sorted {
        gap = gap / k;
        if gap <= 1 {
            gap = 1;
            is_not_sorted = false;
        }

        let mut count = 0;
        while count + gap < len {
            if tab[count] > tab[count+gap] {
                tab.swap(count, count + gap);
                is_not_sorted = true
            }
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comb_sort_test() {
        let mut array = [1, 7, 8, 6, 4];
        comb_sort(&mut array, 5, 2);
        assert!(is_sorted(&array, 5));
    }
}
