fn fib100() -> Vec<u128> {
    let mut v: Vec<u128> = vec![0, 1];
    v.reserve(100);

    for i in 2..100 {
        v.push(v[i - 1] + v[i - 2]);
    }
    v
}

#[test]
fn fibs() {
    let fibs = fib100();
    assert_eq!(fibs.len(), 100);
    assert!(vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        .into_iter()
        .eq(fibs.into_iter().take(10)));
}
