pub fn search<T: Eq>(arr: &[T], elem: &T) -> Option<usize> {
    for (i, el) in arr.iter().enumerate() {
        if el == elem {
            return Option::Some(i);
        }
    }
    return Option::None;
}
