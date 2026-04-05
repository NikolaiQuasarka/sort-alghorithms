pub trait QuickSort<T> {
    fn fast_sort<F>(&mut self, _compare: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
    }
}

impl<T> QuickSort<T> for &mut [T] {
    fn fast_sort<F>(&mut self, compare: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        // fast_sort_impl(slice, &compare);
        if self.len() <= 1 {
            return;
        }

        let mid = self.len() / 2;
        self.swap(mid, self.len() - 1);
        let pivot = self.len() - 1;

        let mut i = 0;
        let mut j = 0;

        while j < pivot {
            if compare(&self[j], &self[pivot]) {
                self.swap(i, j);
                i += 1;
            }
            j += 1;
        }

        self.swap(i, pivot);

        // fast_sort(&mut self[..i], compare);
        // fast_sort(&mut self[i + 1..], compare);
        (&mut self[..i]).fast_sort(compare);
        (&mut self[i + 1..]).fast_sort(compare);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    fn numbers_compare(n1: &u64, n2: &u64) -> bool {
        n1 < n2
    }
    #[test]
    fn numbers_sort() {
        let mut numbers = vec![3, 5, 76, 87, 54, 4];
        numbers.as_mut_slice().fast_sort(&numbers_compare);

        assert_eq!(vec![3, 4, 5, 54, 76, 87], numbers)
    }

    #[test]
    fn large_list_sort() {
        let mut rng = rand::rng();
        let mut list = (0..99_999_999u64).collect::<Vec<u64>>();

        list.shuffle(&mut rng);

        // list.iter().for_each(|el| println!("{}", el));

        list.as_mut_slice().fast_sort(&numbers_compare);

        // list.iter().for_each(|el| println!("{}", el));
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

        list.as_mut_slice().fast_sort(&numbers_compare);

        list.iter().for_each(|el| println!("{}", el));
    }
}
