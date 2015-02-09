use std::num::Float;
use std::borrow::ToOwned;

pub fn sort(og_vec: Vec<usize>) -> Vec<usize> {
    let vec: Vec<usize> = og_vec.to_owned();
    sort_with_pivot(vec, 0us)
}

pub fn sort_with_pivot(vec: Vec<usize>, custom_pivot: usize) -> Vec<usize> {
    if vec.len() <= 1us {
        return vec
    }

    let result = partition(vec, custom_pivot);
    let pivot: Vec<usize> = result.pivot;
    let mut left: Vec<usize> = sort(result.left);
    let right: Vec<usize> = sort(result.right);

    left.push_all(&pivot);
    left.push_all(&right);
    left
}

fn partition(og_vec: Vec<usize>, custom_pivot: usize) -> Partition {
    if og_vec.len() <= 1us {
        return Partition {
            left: Vec::new(),
            pivot: og_vec,
            right: Vec::new(),
        }
    }

    let mut vec: Vec<usize> = og_vec.to_owned();

    if custom_pivot != 0us {
        vec.swap(0us, custom_pivot);
    }

    let pivot = 0us;
    let mut i = 1us;
    let mut j = 1us;
    while j < vec.len() {
        if vec[j] < vec[pivot] {
            if i != j {
                vec.swap(i, j);
            }
            j += 1us;
            i += 1us;
        } else {
            j += 1us;
        }
    }
    vec.swap(pivot, i - 1us);

    return Partition {
        pivot: vec![vec[i-1us]],
        left: vec.iter().enumerate().filter(|&(idx, &_n)| idx < i - 1us).map(|(_idx, &n)| n).collect(),
        right: vec.iter().enumerate().filter(|&(idx, &_n)| idx > i - 1us).map(|(_idx, &n)| n).collect(),
    }
}

pub fn count_comparisons(vec: Vec<usize>) -> (Vec<usize>, usize) {
    count_comparisons_with_pivot_and_count(vec, 0is, 0us)
}

pub fn count_comparisons_with_pivot(vec: Vec<usize>, custom_pivot: isize) -> (Vec<usize>, usize) {
    count_comparisons_with_pivot_and_count(vec, custom_pivot, 0us)
}

pub fn count_comparisons_with_pivot_and_count(og_vec: Vec<usize>, custom_pivot: isize, count: usize) -> (Vec<usize>, usize) {
    let vec = og_vec.to_owned();
    if vec.len() <= 1us {
        return (vec, count)
    }

    let total = count + vec.len() - 1us;

    let pivot_idx =
        if custom_pivot == -1is {
            vec.len() - 1us
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

pub fn count_median_comparisons(vec: Vec<usize>) -> (Vec<usize>, usize) {
    count_median_comparisons_with_count(vec, 0us)
}

pub fn count_median_comparisons_with_count(vec: Vec<usize>, count: usize) -> (Vec<usize>, usize) {
    if vec.len() <= 1us {
        return (vec, count)
    }

    let total = count + vec.len() - 1us;

    let result = median_partition(vec);

    let pivot = result.pivot;
    let (mut left, count1) = count_median_comparisons_with_count(result.left, total);
    let (right, count2) = count_median_comparisons_with_count(result.right, count1);

    left.push_all(&pivot);
    left.push_all(&right);
    (left, count2)
}

fn median_partition(og_vec: Vec<usize>) -> Partition {
    if og_vec.len() <= 1us {
        return Partition {
            left: Vec::new(),
            pivot: og_vec,
            right: Vec::new(),
        }
    }

    let mut vec: Vec<usize> = og_vec.to_owned();

    // let mut pivot = 0us;
    let first = vec[0us];
    let last = *vec.last().unwrap();
    let middle_idx = (vec.len() as f32/2f32).ceil() as usize - 1us;
    let middle = vec[middle_idx];

    let mut pivot =
        if first <= middle && middle <= last {
            middle_idx
        } else if last <= middle && middle <= first {
            middle_idx
        } else if last <= first && first <= middle {
            0us
        } else if middle <= first && first <= last {
            0us
        } else { // last
            vec.len() - 1us
        };

    vec.swap(0us, pivot);

    pivot = 0us;
    let mut i = 1us;
    let mut j = 1us;
    while j < vec.len() {
        if vec[j] < vec[pivot] {
            if i != j {
                vec.swap(i, j);
            }
            j += 1us;
            i += 1us;
        } else {
            j += 1us;
        }
    }
    vec.swap(pivot, i - 1us);

    return Partition {
        pivot: vec![vec[i-1us]],
        left: vec.iter().enumerate().filter(|&(idx, &_n)| idx < i - 1us).map(|(_idx, &n)| n).collect(),
        right: vec.iter().enumerate().filter(|&(idx, &_n)| idx > i - 1us).map(|(_idx, &n)| n).collect(),
    }
}

struct Partition {
    left: Vec<usize>,
    pivot: Vec<usize>,
    right: Vec<usize>,
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
        let vec = vec![8us, 3, 2, 10, 6, 5, 1, 9, 4, 7];
        let expected = (1us..11us).collect();
        assert_eq!(sort(vec), expected);
    }

    #[test]
    pub fn with_no_pivot_specified_it_sorts_an_input_vector_of_100_integers() {
        let mut vec: Vec<usize> = (1us..101us).collect();
        thread_rng().shuffle(&mut vec);
        let expected = (1us..101us).collect();
        assert_eq!(sort(vec), expected);
    }

    #[test]
    pub fn with_no_pivot_specified_it_sorts_an_input_vector_of_1000_integers() {
        let mut vec: Vec<usize> = (1us..1001us).collect();
        thread_rng().shuffle(&mut vec);
        let expected = (1us..1001us).collect();
        assert_eq!(sort(vec), expected);
    }

    #[test]
    pub fn with_a_custom_pivot_it_sorts_an_input_vector_of_1000_integers() {
        let mut vec: Vec<usize> = (1us..1001us).collect();
        thread_rng().shuffle(&mut vec);
        let expected = (1us..1001us).collect();
        assert_eq!(sort(vec), expected);
    }

    #[test]
    pub fn with_no_custom_pivot_it_counts_comparisons() {
        let vec = vec![2us, 1];
        assert_eq!(count_comparisons(vec), (vec![1us, 2], 1us));
    }

    #[test]
    pub fn with_a_custom_pivot_it_counts_comparisons() {
        let vec = vec![2us, 1];
        assert_eq!(count_comparisons_with_pivot(vec, 1is), (vec![1us, 2], 1us));
    }

    #[test]
    pub fn count_median_comparisons_with_ten_txt_test_input() {
        let mut file = BufferedReader::new(File::open(&Path::new("ten.txt")));
        let vec: Vec<usize> = file.lines().enumerate().map(|(i, x)| {
            x.unwrap()
            .trim()
            .parse::<usize>()
            .unwrap_or_else(|err| {
                panic!("Unable to parse int at line:{}: {}", i + 1us, err)
            })
        }).collect();
        assert_eq!(count_median_comparisons(vec).1, 21us);
    }

    #[test]
    pub fn count_median_comparisons_with_hundred_txt_test_input() {
        let mut file = BufferedReader::new(File::open(&Path::new("hundred.txt")));
        let vec: Vec<usize> = file.lines().enumerate().map(|(i, x)| {
            x.unwrap()
            .trim()
            .parse::<usize>()
            .unwrap_or_else(|err| {
                panic!("Unable to parse int at line:{}: {}", i + 1us, err)
            })
        }).collect();
        assert_eq!(count_median_comparisons(vec).1, 518us);
    }

    #[test]
    pub fn count_median_comparisons_with_thousand_txt_test_input() {
        let mut file = BufferedReader::new(File::open(&Path::new("thousand.txt")));
        let vec: Vec<usize> = file.lines().enumerate().map(|(i, x)| {
            x.unwrap()
            .trim()
            .parse::<usize>()
            .unwrap_or_else(|err| {
                panic!("Unable to parse int at line:{}: {}", i + 1us, err)
            })
        }).collect();
        assert_eq!(count_median_comparisons(vec).1, 8921us);
    }
}
