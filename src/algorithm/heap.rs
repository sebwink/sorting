#[inline(always)]
fn left(i: usize) -> usize {
    2 * i + 1
}

#[inline(always)]
fn right(i: usize) -> usize {
    2 * (i + 1)
}

fn heapify<T>(elems: &mut [T], size: usize, mut i: usize)
where T: PartialOrd {
    let mut l = left(i);
    let mut r = right(i);
    let mut largest = i;
    loop {
        if l < size && elems[l] > elems[i] {
            largest = l;
        }
        if r < size && elems[r] > elems[largest] {
            largest = r;
        }
        if i == largest {
            break;
        }
        elems.swap(i, largest);
        i = largest;
        l = left(i);
        r = right(i);
    }
}

pub fn sort<T>(elems: &mut [T])
where T: PartialOrd {
    let mut size = elems.len();
    // BUILD-HEAP
    for i in 0..size >> 1 {
        heapify(elems, size, i);
    }
    // HEAP-SORT
    for i in (1..size).rev() {
        elems.swap(0, i);
        size -= 1;
        heapify(elems, size, 0); 
    }
}
