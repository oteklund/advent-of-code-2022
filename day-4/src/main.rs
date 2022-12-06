use std::fs;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(text) => text,
        Err(error) => panic!("Failed to read file: {:?}", error),
    };

    let split: Vec<&str> = input.split("\n").filter(|line| line.len() > 0).collect();

    let mut amt_contained_completely = 0;
    let mut amt_contained_partially = 0;

    for pair in split {
        if one_contains_other_completely(&pair) {
            amt_contained_completely += 1;
        }

        if one_contains_other_partially(&pair) {
            amt_contained_partially += 1;
        }
    }

    println!("totally contained: {amt_contained_completely}, partially overlapping: {amt_contained_partially}");
}

fn one_contains_other_completely(pair: &str) -> bool {
    let (first_range, second_range) = pair.split_once(",").unwrap();

    let (first_begin, first_end) = first_range.split_once("-").unwrap();
    let first_values: Vec<u32> =
        (first_begin.parse::<u32>().unwrap()..=first_end.parse::<u32>().unwrap()).collect();

    let (second_begin, second_end) = second_range.split_once("-").unwrap();
    let second_values: Vec<u32> =
        (second_begin.parse().unwrap()..=second_end.parse().unwrap()).collect();

    let mut first_contained_in_second = true;
    let mut second_contained_in_first = true;

    for ele in &first_values {
        if second_values.contains(&ele) == false {
            first_contained_in_second = false;
            break;
        }
    }

    for ele in &second_values {
        if first_values.contains(&ele) == false {
            second_contained_in_first = false;
            break;
        }
    }

    return first_contained_in_second || second_contained_in_first;
}

fn one_contains_other_partially(pair: &str) -> bool {
    let (first_range, second_range) = pair.split_once(",").unwrap();

    let (first_begin, first_end) = first_range.split_once("-").unwrap();
    let (second_begin, second_end) = second_range.split_once("-").unwrap();

    let first_values: Vec<u32> =
        (first_begin.parse::<u32>().unwrap()..=first_end.parse::<u32>().unwrap()).collect();
    let second_values: Vec<u32> =
        (second_begin.parse::<u32>().unwrap()..=second_end.parse::<u32>().unwrap()).collect();

    for ele in &first_values {
        if second_values.contains(&ele) {
            return true;
        }
    }

    for ele in &second_values {
        if first_values.contains(&ele) {
            return true;
        }
    }

    return false;
}
