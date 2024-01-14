use algorithms::bubble_sort;

#[test]
fn test_bubble_sort() {
    let mut slice = [5, 4, 3, 2, 1];

    bubble_sort(&mut slice);

    assert_eq!(slice, [1, 2, 3, 4, 5]);
}