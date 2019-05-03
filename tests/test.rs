extern crate sorting;

use std::cmp::Ordering;

fn test_inplace_algorithm<F>(sort: F)
where F: Fn(&mut [i32]) {
    let mut arr = vec![3, 45, 2, 5, 13, 10, 4];
    sort(&mut arr);
    assert_eq!(arr, vec![2, 3, 4, 5, 10, 13, 45]);
    arr = vec![31, 41, 59, 26, 41, 58];
    sort(&mut arr);
    assert_eq!(arr, vec![26, 31, 41, 41, 58, 59]);
}

#[derive(Clone, Copy, Debug)]
struct Data {
    key: i32,
    stuff: f64,
}

impl Data {
    fn new(key: i32, stuff: f64) -> Self {
        Self {
            key,
            stuff,
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.stuff == other.stuff
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

fn inplace_algorithm_seems_to_be_stable<F>(sort: F)
where F: Fn(&mut [Data]) {
    let mut arr = vec![
        Data::new(42, 3.141),
        Data::new(13, 2.7),
        Data::new(1, 1.1111),
        Data::new(42, 3.140),
    ];
    sort(&mut arr);
    assert_eq!(
        arr,
        vec![
            Data::new(1, 1.1111),
            Data::new(13, 2.7),
            Data::new(42, 3.141),
            Data::new(42, 3.140),
        ],
    );
}

fn empty_input_does_not_blow_up<F>(sort: F)
where F: Fn(&mut [i32]) {
    let mut arr: Vec<i32> = Vec::new();
    sort(&mut arr);
}

#[cfg(test)]
mod test {
    use super::*;
    
    mod insertion_sort {
        use super::*;
        use sorting::algorithm::insertion;
        #[test]
        fn works() {
            test_inplace_algorithm(insertion::sort);
        }
        #[test]
        fn seems_to_be_stable() {
            inplace_algorithm_seems_to_be_stable(insertion::sort);
        }
        #[test]
        fn does_not_blow_up_on_empty_input() {
            empty_input_does_not_blow_up(insertion::sort);
        }
    }
    
    mod selection_sort {
        use super::*;
        use sorting::algortihm::selection;
        #[test]
        fn works() {
            test_inplace_algorithm(selection::sort);
        }
        #[test]
        fn seems_to_be_stable() {
            inplace_algorithm_seems_to_be_stable(selection::sort);
        }
        #[test]
        fn does_not_blow_up_on_empty_input() {
            empty_input_does_not_blow_up(selection::sort);
        }
    }
    
    mod bubble_sort {
        use super::*;
        use sorting::algortihm::bubble;
        #[test]
        fn works() {
            test_inplace_algorithm(bubble::sort);
        }
        #[test]
        fn seems_to_be_stable() {
            inplace_algorithm_seems_to_be_stable(bubble::sort);
        }
        #[test]
        fn does_not_blow_up_on_empty_input() {
            empty_input_does_not_blow_up(bubble::sort);
        }
    }
    
    mod merge_sort {
        use super::*;
        use sorting::algortihm::merge;
        #[test]
        fn works() {
            test_inplace_algorithm(merge::sort);
        }
        #[test]
        fn seems_to_be_stable() {
            inplace_algorithm_seems_to_be_stable(merge::sort);
        }
        #[test]
        fn does_not_blow_up_on_empty_input() {
            empty_input_does_not_blow_up(merge::sort);
        }
    }
    
    mod heap_sort {
        use super::*;
        use sorting::algortihm::heap;
        #[test]
        fn works() {
            test_inplace_algorithm(heap::sort);
        }
        #[test]
        fn seems_to_be_stable() {
            inplace_algorithm_seems_to_be_stable(heap::sort);
        }
        #[test]
        fn does_not_blow_up_on_empty_input() {
            empty_input_does_not_blow_up(heap::sort);
        }
    }
}
