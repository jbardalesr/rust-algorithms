#[derive(Debug)]
const LENGTH:usize = 17;
struct Array {
    list:[i32; LENGTH],
    heap_size:usize
}

fn max_heapify(array:&mut Array, i:usize){
    let l = 2*i;
    let r = 2*i + 1;
    let mut largest;

    if l <= array.heap_size && array.list[l] > array.list[i] {
        largest = l;
    } else {
        largest = i;
    }

    if r <= array.heap_size && array.list[r] > array.list[largest] {
        largest = r;
    }

    if i != largest {
        array.list.swap(largest, i);
        max_heapify(array, largest);
    }
}

fn build_max_heap(array: &mut Array){
    array.heap_size = LENGTH - 1;
    for i in (0..(LENGTH - 1)/2 + 1).rev() {
        max_heapify(array, i);        
    }
}

fn heapsort(array: &mut Array) {
    build_max_heap(array);
    for i in (1..LENGTH).rev() {
        array.list.swap(0, i);
        array.heap_size -= 1;
        max_heapify(array, 0);
    }
}

fn main() {
    let mut array = Array {
        list:[9, 15, -4, 0, 6, 3, 2, -1, 10, 98, 1, -10, 15, 90, 68, 14, 20],
        heap_size: 0
    };

    //heapsort(&mut array);
    println!("{:?}", array.list);
}