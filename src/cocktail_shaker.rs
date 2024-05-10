use crate::is_sorted;

fn up<T: PartialOrd>(tab: &mut [T], len: usize) {
    for i in 1..len {
        if tab[i] < tab[i-1] {
            tab.swap(i, i-1);
        }
    };
}

fn down<T: PartialOrd>(tab: &mut [T], len: usize) {
    for i in len..1 {
        if tab[i] < tab[i-1] {
            tab.swap(i, i-1);
        }
    };
}

fn cocktail_sort<T: PartialOrd>(tab: &mut [T], len: usize) {
    let mut sort = is_sorted(tab, len);
    while !sort {
        up(tab, len);
        down(tab, len);
        sort = is_sorted(tab, len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cocktail_sort_test() {
        let mut array = [1, 7, 8, 6, 4];
        cocktail_sort(&mut array, 5);
        assert!(is_sorted(&array, 5));
    }
}

