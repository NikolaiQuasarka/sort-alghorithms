pub trait QuickSort<T: PartialOrd> {
    fn fast_sort(&mut self) {}
}

impl<T: PartialOrd> QuickSort<T> for [T] {
    fn fast_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        let mid = self.len() / 2;
        self.swap(mid, self.len() - 1);
        let pivot = self.len() - 1;

        let mut i = 0;
        let mut j = 0;

        while j < pivot {
            if &self[j] < &self[pivot] {
                self.swap(i, j);
                i += 1;
            }
            j += 1;
        }

        self.swap(i, pivot);

        self[..i].fast_sort();
        self[i + 1..].fast_sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    // fn numbers_compare(n1: &u64, n2: &u64) -> bool {
    //     n1 < n2
    // }
    #[test]
    fn numbers_sort() {
        let mut numbers = vec![3, 5, 76, 87, 54, 4];
        numbers.fast_sort();

        assert_eq!(vec![3, 4, 5, 54, 76, 87], numbers)
    }

    #[test]
    fn large_list_sort() {
        let mut rng = rand::rng();
        let mut list = (0..999_999u64).collect::<Vec<u64>>();

        list.shuffle(&mut rng);

        // list.iter().for_each(|el| println!("{}", el));

        list.fast_sort();

        // list.iter().for_each(|el| println!("{}", el));
    }

    #[test]
    fn reserved_list() {
        let mut list = (0..999_999).rev().collect::<Vec<u64>>();

        list.fast_sort();

        assert_eq!((0..999_999).collect::<Vec<_>>(), list)
    }

    #[test]
    fn opposite_sort() {
        let mut list = vec![];

        let end = 999_999;

        for i in 0..=end {
            match i % 2 {
                0 => list.push(i),
                1 => list.push(end - i),
                _ => unreachable!(),
            }
        }

        list.iter().for_each(|el| println!("{}", el));

        list.fast_sort();

        list.iter().for_each(|el| println!("{}", el));
    }
}
