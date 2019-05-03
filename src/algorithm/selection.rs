fn min_index<T>(arr: &[T]) -> usize
where T: PartialOrd {
    let mut min = &arr[0];
    let mut index = 0;
    for (i, elem) in arr.iter().enumerate().skip(1) {
        if elem < min {
            min = elem;
            index = i;
        }
    }
    index
}

pub fn sort<T>(arr: &mut [T]) 
where T: PartialOrd {
    for i in 0..arr.len() {
        let arri = &mut arr[i..];
        let mi = min_index(arri);
        if mi > 0 {
            arri.swap(0, mi);
        }
    }
}
