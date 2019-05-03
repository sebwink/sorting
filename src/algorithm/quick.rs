extern crate rand;

pub mod partition {
    pub fn lomuto<T>(arr: &mut [T], p: usize, r: usize) -> usize
    where T: PartialOrd {
        let mut i = p;
        for j in p..r {
            if arr[j] <= arr[r - 1] {
                arr.swap(i, j);
                i += 1;
            }
        }
        if i < r {
            arr.swap(i, r - 1);
        }
        i - 1
    }

    pub fn hoare<T>(arr: &mut [T], p: usize, r: usize) -> usize
    where T: PartialOrd {
        let mut i = p;
        let mut j = r - 1;
        loop {
            while arr[j] > arr[r - 1] {
                j -= 1;
            }
            while arr[i] < arr[r - 1] {
                i += 1;
            }
            if i < j {
                arr.swap(i, j);
            } else {
                return j;
            }
        }
    }
}

pub mod pivot {
    use rand::{Rng, thread_rng};

    pub fn last(_p: usize, r: usize) -> usize {
        r - 1
    }

    pub fn random(p: usize, r: usize) -> usize {
        thread_rng().gen_range(p, r)
    }
}

pub fn partition<T>(
    arr: &mut [T],
    p: usize,
    r: usize,
    pivot: fn(usize, usize) -> usize,
    partitioner: fn(&mut [T], usize, usize) -> usize,
) -> usize 
where T: PartialOrd {
    let i = pivot(p, r);
    arr.swap(i, r - 1);
    partitioner(arr, p, r)
}

fn quicksort<T>(
    arr: &mut [T],
    p: usize,
    r: usize,
    pivot: fn(usize, usize) -> usize,
    partitioner: fn(&mut [T], usize, usize) -> usize,
) 
where T: PartialOrd {
    if p < r {
        let q = partition(arr, p, r, pivot, partitioner);
        quicksort(arr, p, q, pivot, partitioner);
        quicksort(arr, q + 1, r, pivot, partitioner);
    }
}

pub fn sort<T>(
    arr: &mut [T],
    pivot: fn(usize, usize) -> usize,
    partitioner: fn(&mut [T], usize, usize) -> usize,
)
where T: PartialOrd {
    let arrlen = arr.len();
    if arrlen != 0 {
        quicksort(arr, 0, arrlen, pivot, partitioner);
    }
}
