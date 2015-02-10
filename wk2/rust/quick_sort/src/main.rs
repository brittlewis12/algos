#![feature(io)]

use std::old_io::BufferedReader;
use std::old_io::File;

mod quick_sort;

fn main() {
    let mut file = BufferedReader::new(File::open(&Path::new("IntegerInput.txt")));
    let integers: Vec<i32> = file.lines().enumerate().map(|(i, x)| {
        x.unwrap()
        .trim()
        .parse::<i32>()
        .unwrap_or_else(|err| {
            panic!("Unable to parse int at line:{}: {}", i + 1, err)
        })
    }).collect();

    // println!("Integers: {:?}", integers);

    let (_, result1) = quick_sort::count_comparisons(integers.clone());
    println!("Comparisons with first: {}", result1);

    let (_, result2) = quick_sort::count_comparisons_with_pivot(integers.clone(), -1);
    println!("Comparisons with last: {}", result2);

    let (_, result3) = quick_sort::count_median_comparisons(integers.clone());
    println!("Comparisons with median-of-three: {}", result3);
}

