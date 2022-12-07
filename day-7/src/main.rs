use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read file");

    let lines: Vec<&str> = input.split("\n").filter(|row| row.len() > 0).collect();

    // Vector of tuples containing directory name and size
    let mut directory_lookup: Vec<(String, u32)> = vec![];

    let mut path_components: Vec<&str> = vec![];

    for line in lines {
        let elements = line.split(" ").collect::<Vec<&str>>();

        let file_size = match elements[0].parse::<u32>() {
            Ok(size) => size,
            Err(_) => 0,
        };

        if file_size > 0 {
            let mut path: String = "".to_owned();

            // Add the file size to all the folders' total size
            for dirname in &path_components {
                path.push_str(dirname.to_owned());

                match directory_lookup
                    .iter()
                    .position(|directory| directory.0 == path)
                {
                    Some(index) => {
                        directory_lookup[index].1 += file_size;
                    }
                    None => directory_lookup.push((path.to_owned(), file_size)),
                };
            }
        }

        if elements[1] == "cd" {
            if elements[2] == ".." {
                path_components.pop();
            } else {
                path_components.push(elements[2]);
            }
        }
    }

    let mut total = 0;

    const LIMIT: u32 = 100_000;

    for (_name, size) in &directory_lookup {
        if size <= &LIMIT {
            total += size;
        }
    }

    println!("Total: {total}");

    const TOTAL_SPACE: u32 = 70_000_000;
    const REQUIRED_SPACE: u32 = 30_000_000;

    let used_space = match directory_lookup.iter().find(|directory| directory.0 == "/") {
        Some(directory) => directory.1,
        None => panic!("Root dir not found"),
    };

    let free_space = TOTAL_SPACE - used_space;

    let space_to_free = REQUIRED_SPACE - free_space;

    println!("Space that needs to be freed: {space_to_free}");

    let large_enough_directories = directory_lookup
        .iter()
        .filter(|directory| directory.1 >= space_to_free);

    let smallest_eligible_directory = large_enough_directories
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    println!(
        "The smallest directory that frees up enough space is {}, with a size of {}",
        smallest_eligible_directory.0, smallest_eligible_directory.1
    );
}
