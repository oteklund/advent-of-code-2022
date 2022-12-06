use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read file");

    // filter out the empty string at the end
    let split = input.split('\n').filter(|entry| entry.len() > 0);

    let entries: Vec<&str> = split.collect();

    solve_part_one(&entries);

    solve_part_two(&entries);
}

fn solve_part_one(entries: &Vec<&str>) {
    let mut total: usize = 0;

    for ele in entries {
        let length = ele.len();

        let (first, second) = ele.split_at(length / 2);

        for ele in first.chars() {
            if second.contains(ele) {
                total += get_char_value(&ele);

                println!("Total after adding: {total}");

                break;
            }
        }
    }

    println!(
        "=====================
| Grand total: {total} |
====================="
    ); // 8243
}

fn solve_part_two(entries: &Vec<&str>) {
    let mut index = 0;

    let mut total = 0;

    loop {
        let group = entries[index..index + 3].to_vec();

        // let group_length = group.len();

        index += 3;

        // println!("group length: {group_length}\ncontent: {:?}", group);

        let badge = find_badge_item(&group).expect("Failed to get badge");

        println!("found badge: {badge}");

        total += get_char_value(&badge);

        if index >= entries.len() {
            break;
        }
    }

    println!("total: {total}"); // 2631
}

#[derive(Debug)]
enum BadgeError {
    NotFound,
}

fn find_badge_item(group: &Vec<&str>) -> Result<char, BadgeError> {
    let first = group[0];
    let second = group[1];
    let third = group[2];

    for char in first.chars() {
        if second.contains(|c| c == char) && third.contains(|c| c == char) {
            return Ok(char);
        }
    }

    return Err(BadgeError::NotFound);
}

fn get_char_value(char: &char) -> usize {
    let letters = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let found_letter = letters.iter().find(|&letter| letter == char).unwrap();

    let found_letter_index = letters.iter().position(|&letter| letter == *char).unwrap();

    let value = found_letter_index + 1;

    println!("Found letter: {found_letter}, which has the value {value}");

    return value;
}
