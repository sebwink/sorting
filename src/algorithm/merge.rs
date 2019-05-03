fn merge<T>(
    arr: &mut [T],
    p: usize,
    q: usize,
    r: usize,
)
where T: PartialOrd + Clone {
    let left = &arr[p..q].to_vec();
    let right = &arr[q..r].to_vec();
    let mut i = 0;
    let mut j = 0;
    for k in p..r {
        if let Some(lefti) = left.get(i) {
            if let Some(rightj) = right.get(j) {
                if lefti <= rightj {
                    arr[k] = lefti.clone();
                    i += 1;
                } else {
                    arr[k] = rightj.clone();
                    j += 1;
                }
            } else {
                arr[k] = lefti.clone();
                i += 1;
            }
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
    }
}

fn mergesort<T>(
    arr: &mut [T],
    p: usize,
    r: usize,
) 
where T: Clone + PartialOrd {
    if p < r - 1 {
        let q = p + r >> 1;    // floor( (p + r) / 2 )
        mergesort(arr, p, q);
        mergesort(arr, q, r);
        merge(arr, p, q, r);
    }
}

pub fn sort<T>(arr: &mut [T])
where T: Clone + PartialOrd {
    let arrlen = arr.len();
    if arrlen != 0 {
        mergesort(arr, 0, arrlen);
    }
}
