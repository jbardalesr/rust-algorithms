fn bubble_sort(arr: &mut [i32]) {
    let n:usize = arr.len();
    loop {
        let mut swapped:bool = false;
        for j in 0..n - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
