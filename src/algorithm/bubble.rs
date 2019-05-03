pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    let arrlen = arr.len();
    for i in 0..arrlen {
        for j in (i + 1..arrlen).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j - 1, j);
            }
        }
    }
}
