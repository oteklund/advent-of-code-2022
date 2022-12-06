use std::fs;

fn main() {
    let input = match fs::read_to_string("input.txt") {
        Ok(input) => input,
        Err(error) => panic!("Failed to read input: {error}"),
    };

    let parsed: String = input.chars().filter(|char| char.is_alphabetic()).collect();

    let chars: Vec<char> = parsed.chars().collect();

    let mut index = 0;

    let mut result = 0;

    loop {
        let mut has_duplicate_char = false;

        let slice = match chars.get(index..index + 14) {
            Some(slice) => slice,
            None => {
                println!("Reached end of list with no match");

                break;
            }
        };

        for char in slice {
            let instances_in_slice: Vec<&char> = slice.iter().filter(|&ele| ele == char).collect();

            if instances_in_slice.len() > 1 {
                has_duplicate_char = true;

                break;
            }
        }

        if has_duplicate_char == false {
            result = index + 14;

            break;
        }

        index += 1;
    }

    println!("The result is: {result}");
}
