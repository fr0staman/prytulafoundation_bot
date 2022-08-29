pub fn split_half(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, ' ');
    let first = splitter.next().unwrap();
    let second = match splitter.next() {
        Some(value) => value,
        None => "",
    };
    (first, second)
}
