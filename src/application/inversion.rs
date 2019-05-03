// INSERTION SORT

fn get_with_insertion_sort<T>(arr: &[T]) -> Vec<(usize, usize)>
where T: PartialOrd + Clone {
    let mut array = arr.to_vec();
    let mut inversions = Vec::new();
    for i in 1..array.len() {
        let ei = array[i].clone();
        let mut j = i;
        while j > 0 && ei < array[j - 1] {
            j -= 1;
            inversions.push((j, i));
            array[j + 1] = array[j].clone();
        }
        array[j] = ei;
    }
    inversions 
}

// MERGE SORT

fn merge<T>(
    arr: &mut [T],
    p: usize,
    q: usize,
    r: usize,
    inversions: &mut Vec<(usize, usize)>,
) where T: PartialOrd + Clone {
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
                    inversions.push((p + i, q + j));
                    arr[k] = rightj.clone();
                    j += 1;
                }
            } else {
                inversions.push((q + i, r - 1));
                arr[k] = lefti.clone();
                i += 1;
            }
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
    }
}

fn merge_sort<T>(
    arr: &mut [T],
    p: usize,
    r: usize,
    inversions: &mut Vec<(usize, usize)>,
)
where T: Clone + PartialOrd {
    if p < r - 1 {
        let q = p + r >> 1;    // floor( (p + r) / 2 )
        merge_sort(arr, p, q, inversions);
        merge_sort(arr, q, r, inversions);
        merge(arr, p, q, r, inversions);
    }
}

fn get_with_merge_sort<T>(arr: &[T]) -> Vec<(usize, usize)>
where T: Clone + PartialOrd {
    let mut inversions = Vec::new();
    let mut array = arr.to_vec();
    let arrlen = array.len();
    if arrlen != 0 {
        merge_sort(&mut array, 0, arrlen, &mut inversions);
    }
    inversions
}


pub mod get {
    use super::*;

    // Get inversions with INSERTION SORT
    pub fn with_insertion_sort<T>(arr: &[T]) -> Vec<(usize, usize)>
    where T: PartialOrd + Clone {
        get_with_insertion_sort(&arr)
    }
    
    // Get inversions with MERGE SORT
    pub fn with_merge_sort<T>(arr: &[T]) -> Vec<(usize, usize)>
    where T: PartialOrd + Clone {
        get_with_merge_sort(&arr)
    }
}

pub mod count {
    use super::get;
    
    // Count inversions with INSERTION SORT
    pub fn with_insertion_sort<T>(arr: &[T]) -> usize
    where T: PartialOrd + Clone {
        get::with_insertion_sort(&arr).len()
    }
    
    // Count inversions with MERGE SORT
    pub fn with_merge_sort<T>(arr: &[T]) -> usize
    where T: PartialOrd + Clone {
        get::with_merge_sort(&arr).len()
    }
}
