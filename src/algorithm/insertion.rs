pub fn sort<T>(arr: &mut [T])
where T: Clone + PartialOrd {
    for i in 1..arr.len() {
        let ei = arr[i].clone();
        let mut j = i;
        while j > 0 && ei < arr[j - 1] {
            j -= 1;
            arr[j + 1] = arr[j].clone();
        }
        arr[j] = ei;
    }
}
