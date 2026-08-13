pub fn find<T: Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    array.as_ref().binary_search(&key).ok()
}
