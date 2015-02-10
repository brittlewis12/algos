use std::num::Float;
use std::borrow::ToOwned;

pub fn sort(og_vec: Vec<i32>) -> Vec<i32> {
    let vec: Vec<i32> = og_vec.to_owned();
    sort_with_pivot(vec, 0)
}

pub fn sort_with_pivot(vec: Vec<i32>, custom_pivot: i32) -> Vec<i32> {
    if vec.len() <= 1 {
        return vec
    }

    let pivot_idx =
        if custom_pivot == -1i32 {
            vec.len() - 1
        } else {
            custom_pivot as usize
        };

    let result = partition(vec, pivot_idx);
    let pivot: Vec<i32> = result.pivot;
    let mut left: Vec<i32> = sort(result.left);
    let right: Vec<i32> = sort(result.right);

    left.push_all(&pivot);
    left.push_all(&right);
    left
}

fn partition(og_vec: Vec<i32>, custom_pivot: usize) -> Partition {
    if og_vec.len() <= 1 {
        return Partition {
            left: Vec::new(),
            pivot: og_vec,
            right: Vec::new(),
        }
    }

    let mut vec: Vec<i32> = og_vec.to_owned();

    if custom_pivot != 0 {
        vec.swap(0, custom_pivot);
    }

    let pivot = 0;
    let mut i = 1;
    let mut j = 1;
    while j < vec.len() {
        if vec[j] < vec[pivot] {
            if i != j {
                vec.swap(i, j);
            }
            j += 1;
            i += 1;
        } else {
            j += 1;
        }
    }
    vec.swap(pivot, i - 1);

    return Partition {
        pivot: vec![vec[i-1]],
        left: vec.iter().enumerate().filter(|&(idx, &_n)| idx < i - 1).map(|(_idx, &n)| n).collect(),
        right: vec.iter().enumerate().filter(|&(idx, &_n)| idx > i - 1).map(|(_idx, &n)| n).collect(),
    }
}

pub fn count_comparisons(vec: Vec<i32>) -> (Vec<i32>, u32) {
    count_comparisons_with_pivot_and_count(vec, 0i32, 0)
}

pub fn count_comparisons_with_pivot(vec: Vec<i32>, custom_pivot: i32) -> (Vec<i32>, u32) {
    count_comparisons_with_pivot_and_count(vec, custom_pivot, 0)
}

pub fn count_comparisons_with_pivot_and_count(og_vec: Vec<i32>, custom_pivot: i32, count: u32) -> (Vec<i32>, u32) {
    let vec = og_vec.to_owned();
    if vec.len() <= 1 {
        return (vec, count)
    }

    let total = count + vec.len() as u32 - 1;

    let pivot_idx =
        if custom_pivot == -1i32 {
            vec.len() - 1
        } else {
            custom_pivot as usize
        };

    let result = partition(vec, pivot_idx);

    let pivot = result.pivot;
    let (mut left, count1) = count_comparisons_with_pivot_and_count(result.left, custom_pivot, total);
    let (right, count2) = count_comparisons_with_pivot_and_count(result.right, custom_pivot, count1);

    left.push_all(&pivot);
    left.push_all(&right);
    (left, count2)
}

pub fn count_median_comparisons(vec: Vec<i32>) -> (Vec<i32>, u32) {
    count_median_comparisons_with_count(vec, 0)
}

pub fn count_median_comparisons_with_count(vec: Vec<i32>, count: u32) -> (Vec<i32>, u32) {
    if vec.len() <= 1 {
        return (vec, count)
    }

    let total = count + vec.len() as u32 - 1;

    let result = median_partition(vec);

    let pivot = result.pivot;
    let (mut left, count1) = count_median_comparisons_with_count(result.left, total);
    let (right, count2) = count_median_comparisons_with_count(result.right, count1);

    left.push_all(&pivot);
    left.push_all(&right);
    (left, count2)
}

fn median_partition(og_vec: Vec<i32>) -> Partition {
    if og_vec.len() <= 1 {
        return Partition {
            left: Vec::new(),
            pivot: og_vec,
            right: Vec::new(),
        }
    }

    let mut vec: Vec<i32> = og_vec.to_owned();

    // let mut pivot = 0;
    let first = vec[0];
    let last = *vec.last().unwrap();
    let middle_idx = (vec.len() as f32/2f32).ceil() as usize - 1;
    let middle = vec[middle_idx];

    let mut pivot =
        if first <= middle && middle <= last {
            middle_idx
        } else if last <= middle && middle <= first {
            middle_idx
        } else if last <= first && first <= middle {
            0
        } else if middle <= first && first <= last {
            0
        } else { // last
            vec.len() - 1
        };

    vec.swap(0, pivot);

    pivot = 0;
    let mut i = 1;
    let mut j = 1;
    while j < vec.len() {
        if vec[j] < vec[pivot] {
            if i != j {
                vec.swap(i, j);
            }
            j += 1;
            i += 1;
        } else {
            j += 1;
        }
    }
    vec.swap(pivot, i - 1);

    return Partition {
        pivot: vec![vec[i-1]],
        left: vec.iter().enumerate().filter(|&(idx, &_n)| idx < i - 1).map(|(_idx, &n)| n).collect(),
        right: vec.iter().enumerate().filter(|&(idx, &_n)| idx > i - 1).map(|(_idx, &n)| n).collect(),
    }
}

struct Partition {
    left: Vec<i32>,
    pivot: Vec<i32>,
    right: Vec<i32>,
}

#[cfg(test)]
mod test {
    extern crate rand;

    use super::*;
    use self::rand::{thread_rng, Rng};
    use std::old_io::BufferedReader;
    use std::old_io::File;

    #[test]
    pub fn with_no_pivot_specified_it_sorts_an_input_vector_of_10_integers() {
        let vec = vec![8, 3, 2, 10, 6, 5, 1, 9, 4, 7];
        let expected = (1..11).collect();
        assert_eq!(sort(vec), expected);
    }

    #[test]
    pub fn with_no_pivot_specified_it_sorts_an_input_vector_of_100_integers() {
        let mut vec: Vec<i32> = (1..101).collect();
        thread_rng().shuffle(&mut vec);
        let expected = (1..101).collect();
        assert_eq!(sort(vec), expected);
    }

    #[test]
    pub fn with_no_pivot_specified_it_sorts_an_input_vector_of_1000_integers() {
        let mut vec: Vec<i32> = (1..1001).collect();
        thread_rng().shuffle(&mut vec);
        let expected = (1..1001).collect();
        assert_eq!(sort(vec), expected);
    }

    #[test]
    pub fn with_a_custom_pivot_it_sorts_an_input_vector_of_1000_integers() {
        let mut vec: Vec<i32> = (1..1001).collect();
        thread_rng().shuffle(&mut vec);
        let expected = (1..1001).collect();
        assert_eq!(sort(vec), expected);
    }

    #[test]
    pub fn with_no_custom_pivot_it_counts_comparisons() {
        let vec = vec![2, 1];
        assert_eq!(count_comparisons(vec), (vec![1, 2], 1));
    }

    #[test]
    pub fn with_a_custom_pivot_it_counts_comparisons() {
        let vec = vec![2, 1];
        assert_eq!(count_comparisons_with_pivot(vec, 1), (vec![1, 2], 1));
    }

    #[test]
    pub fn count_median_comparisons_with_ten_txt_test_input() {
        let mut file = BufferedReader::new(File::open(&Path::new("ten.txt")));
        let vec: Vec<i32> = file.lines().enumerate().map(|(i, x)| {
            x.unwrap()
            .trim()
            .parse::<i32>()
            .unwrap_or_else(|err| {
                panic!("Unable to parse int at line:{}: {}", i + 1, err)
            })
        }).collect();
        assert_eq!(count_median_comparisons(vec).1, 21);
    }

    #[test]
    pub fn count_median_comparisons_with_hundred_txt_test_input() {
        let mut file = BufferedReader::new(File::open(&Path::new("hundred.txt")));
        let vec: Vec<i32> = file.lines().enumerate().map(|(i, x)| {
            x.unwrap()
            .trim()
            .parse::<i32>()
            .unwrap_or_else(|err| {
                panic!("Unable to parse int at line:{}: {}", i + 1, err)
            })
        }).collect();
        assert_eq!(count_median_comparisons(vec).1, 518);
    }

    #[test]
    pub fn count_median_comparisons_with_thousand_txt_test_input() {
        let mut file = BufferedReader::new(File::open(&Path::new("thousand.txt")));
        let vec: Vec<i32> = file.lines().enumerate().map(|(i, x)| {
            x.unwrap()
            .trim()
            .parse::<i32>()
            .unwrap_or_else(|err| {
                panic!("Unable to parse int at line:{}: {}", i + 1, err)
            })
        }).collect();
        assert_eq!(count_median_comparisons(vec).1, 8921);
    }
}
