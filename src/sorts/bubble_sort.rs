pub fn bubble_sort(slice: &mut [i32]) {
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 1..slice.len() {
            if slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}