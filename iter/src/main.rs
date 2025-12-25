fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    println!("total: {}", total);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + 1);

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 9);
}
