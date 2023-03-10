fn largest_number<T>(slice: &mut [T])
where
    T: std::fmt::Display,
{
    slice.sort_by(|a, b| {
        format!("{a}")
            .partial_cmp(&format!("{b}"))
            .expect("it works bro")
            .reverse()
    })
}

#[test]
fn is_it_really_largest() {
    let mut xs = [50, 2, 1, 9];
    largest_number(&mut xs);
    assert_eq!(xs, [9, 50, 2, 1])
}
