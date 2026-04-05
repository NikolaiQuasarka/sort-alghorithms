pub fn fast_sort<T, F>(slice: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    if slice.is_empty() {
        return;
    }

    let mid = slice.len() / 2;
    let key_element = &slice[mid];

    let mut left: Vec<&T> = Vec::new();
    let mut right: Vec<&T> = Vec::new();

    for element in slice.iter() {
        if compare(element, key_element) {
            right.push(element);
        } else {
            left.push(element);
        }
    }
}
