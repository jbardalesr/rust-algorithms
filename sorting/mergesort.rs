fn merge_sort(vec: &mut Vec<i32>, p: usize, r: usize) {
    if p < r {
        let mid = (p + r) / 2;
        merge_sort(vec, p, mid);
        merge_sort(vec, mid + 1, r);

        //copy from slice
        let mut vec_left = Vec::from(&vec[p..mid + 1]);
        let mut vec_right = Vec::from(&vec[mid + 1..r + 1]);

        vec_left.push(i32::MAX);
        vec_right.push(i32::MAX);

        let mut i: usize = 0;
        let mut j: usize = 0;

        for k in p..r + 1 {
            if vec_left[i] <= vec_right[j] {
                vec[k] = vec_left[i];
                i += 1;
            } else {
                vec[k] = vec_right[j];
                j += 1;
            }
        }
    }
}
