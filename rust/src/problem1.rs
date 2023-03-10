fn sum_using_for<It, T>(it: It) -> Option<T>
where
    It: Iterator<Item = T>,
    T: std::ops::Add<Output = T>,
{
    // Rust doesn't have generic way to get 0 of given type so we have to do this dance
    let mut acc = None;
    for c in it {
        acc = Some(match acc {
            Some(p) => p + c,
            None => c,
        });
    }
    return acc;
}

fn sum_using_recursion<It, T>(mut it: It) -> Option<T>
where
    It: Iterator<Item = T>,
    T: std::ops::Add<Output = T>,
{
    it.next().map(|c| match sum_using_recursion(it) {
        Some(p) => p + c,
        None => c,
    })
}

#[test]
fn sums() {
    assert_eq!(sum_using_for(vec![1, 2, 3, 4].into_iter()), Some(10));
    assert_eq!(sum_using_recursion(vec![1, 2, 3, 4].into_iter()), Some(10));

    assert_eq!(sum_using_for(Vec::<i32>::new().into_iter()), None);
    assert_eq!(sum_using_recursion(Vec::<i32>::new().into_iter()), None);
}
