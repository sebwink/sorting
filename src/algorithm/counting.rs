pub fn sort(arr: &[usize], sarr: &mut [usize], k: usize) { 
    let arrlen = arr.len();
    let mut cnt: Vec<usize> = (0..k + 1).map(|_| { 0 }).collect();
    let mut arri;
    for i in 0..arrlen {
        arri = arr[i];
        cnt[arri] = cnt[arri] + 1;
    }
    for i in 1..k + 1 {
        cnt[i] = cnt[i] + cnt[i - 1]; 
    }
    for i in (0..arrlen).rev() {
        arri = arr[i];
        cnt[arri] = cnt[arri] - 1;
        sarr[cnt[arri]] = arri;
    }
}
