fn partition(array: &mut [i32], p: usize, r: usize)->usize {
    let mut i:usize = p;
    let pivot = array[r];
    
    for j in p..r {
        if array[j] < pivot {
            if j != p { // avoids i32 type in Rust for index of arrays
                i = i + 1;
            }
            if j!= i {
                array.swap(i, j)
            }
        }
    }
    array.swap(i + 1, r);
    return i + 1;
}

fn quicksort(array: &mut [i32], p: usize, r: usize) {
    if p < r {
        let q:usize = partition(array, p, r);
        quicksort(array, p, q - 1);
        quicksort(array, q + 1, r);
    }
}

fn main() {
    let mut array: [i32;7] = [-2, -8, 55, 31, 1, 81, 4];
    quicksort(&mut array, 0, 6);
    println!("{:?}", array);
}