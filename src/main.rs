use std::fmt::Debug;

use sorting::algorithm::{
    insertion,
    selection,
    bubble,
    merge,
    heap,
    quick,
    intro,
    counting,
};

use sorting::application::{
    inversion,
};

fn example_with<T>(sort: fn(&mut [T]), input: &Vec<T>, prefix: &str)
where T: PartialOrd + Clone + Debug {
    let mut arr = input.clone();
    sort(&mut arr);
    println!("{}: {:?}", prefix, arr);
}

fn example_with_quicksort<T>(
    input: &Vec<T>,
    pivot: fn(usize, usize) -> usize,
    partitioner: fn(&mut [T], usize, usize) -> usize,
    prefix: &str,
)
where T: PartialOrd + Clone + Debug {
    let mut arr = input.clone();
    quick::sort(&mut arr, pivot, partitioner);
    println!("{}: {:?}", prefix, arr);
}

fn main() {
    println!("\n");
    // SORTING
    let arr = vec![3, 45, 2, 5, 13, 10, 4];
    println!("> Sorting {:?} with ...\n", arr);
    example_with(insertion::sort, &arr, "  ... INSERTION SORT"); 
    example_with(selection::sort, &arr, "  ... SELECTION SORT"); 
    example_with(bubble::sort, &arr, "  ... BUBBLE SORT"); 
    example_with(merge::sort, &arr, "  ... MERGE SORT");
    example_with(heap::sort, &arr, "  ... HEAP SORT");
    example_with_quicksort(
        &arr, quick::pivot::last, quick::partition::lomuto,
        "  ... QUICK SORT (with non-random pivot and Lomuto partition)",
    );
    example_with_quicksort(
        &arr, quick::pivot::random, quick::partition::lomuto,
        "  ... QUICK SORT (with random pivot and Lomuto partition)",
    );
    example_with_quicksort(
        &arr, quick::pivot::last, quick::partition::hoare,
        "  ... QUICK SORT (with non-random pivot and Hoare partition)",
    );
    example_with_quicksort(
        &arr, quick::pivot::random, quick::partition::hoare,
        "  ... QUICK SORT (with random pivot and Hoare partition)",
    );
    example_with(intro::sort, &arr, "  ... INTRO SORT");

    let arr: Vec<usize> = vec![4, 2, 1, 5, 4, 2, 1, 0, 7, 2];
    let mut sarr: Vec<usize> = (0..arr.len()).collect();
    counting::sort(&arr, &mut sarr, 7);
    println!("> Sorting {:?} with ...", arr);
    println!("  ... COUNTING SORT: {:?}", sarr);
    
    // COUNTING INVERSIONS
    let arr = vec![2, 3, 8, 6, 1];
    println!("\n> Calculating inversions of {:?} with ...\n", arr);
    println!(
        "  ... INSERTION SORT: {:?} (#inversions: {})",
        inversion::get::with_insertion_sort(&arr),
        inversion::count::with_insertion_sort(&arr),
    );
    println!(
        "  ... MERGE SORT: {:?} (#inversions: {})",
        inversion::get::with_merge_sort(&arr),
        inversion::count::with_merge_sort(&arr),
    );
    //
    println!("\n");
}
