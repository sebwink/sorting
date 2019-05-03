use super::{
    quick,
    heap,
};

pub fn introsort<T>(
    arr: &mut [T],
    p: usize,
    r: usize,
    pivot: fn(usize, usize) -> usize,
    partitioner: fn(&mut [T], usize, usize) -> usize,
    maxdepth: usize,
) 
where T: PartialOrd {
    if maxdepth == 0 {
        heap::sort(arr);
    } else {
        if p < r {
            let q = quick::partition(arr, p, r, pivot, partitioner);
            introsort(arr, p, q, pivot, partitioner, maxdepth - 1);
            introsort(arr, q + 1, r, pivot, partitioner, maxdepth - 1);
        }
    }
}

pub fn sort<T>(arr: &mut [T])
where T: PartialOrd {
    let arrlen = arr.len();
    let maxdepth = 2 * ( (arrlen as f64).log2() as usize );
    introsort(
        arr,
        0,
        arrlen,
        quick::pivot::random,
        quick::partition::lomuto,
        maxdepth,
    );
}
