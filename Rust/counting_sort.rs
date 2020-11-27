const N:usize = 9;

fn max_item(array: &[usize])->usize {
    let mut max = array[0];
    for i in 1..N {
        if max < array[i] {
            max = array[i];
        }
    }
    return max;
}

fn counting_sort(array: &[usize])->[usize;N] {
    let k = max_item(&array);
    let mut c_vec = Vec::new();
    let mut b:[usize; N] = [0;N];
    for _ in 0..k+1 {
        c_vec.push(0);
    }
    
    for j in 0..N {
        c_vec[array[j]]+=1;
    }
    
    for i in 1..k+1 {
        c_vec[i]+=c_vec[i - 1];
    }
    
    for j in (0..N).rev() {
        b[c_vec[array[j]] - 1] = array[j];
        c_vec[array[j]] -=1;
    }
    
    return b;
}

fn main() {
    let array: [usize; N] = [0, 2, 0, 1, 6, 4, 6, 1, 3];
    let b: [usize; N] = counting_sort(&array);
    println!("{:?}", b);
}
