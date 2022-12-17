fn main() {
    let input = match std::fs::read_to_string("input.txt") {
        Ok(text) => text,
        Err(err) => panic!("Failed to read input file: {}", err),
    };

    let rows: Vec<String> = input
        .split("\n")
        .filter(|row| row.len() > 0)
        .map(str::to_string)
        .collect();

    let mut columns: Vec<String> = vec![];

    for row in &rows {
        let chars: Vec<char> = row.chars().collect();
        for i in 0..chars.len() {
            let tree = chars[i];
            match columns.get(i) {
                Some(column) => {
                    columns[i] = format!("{}{}", column, tree);
                }
                None => {
                    columns.push(tree.to_string());
                }
            };
        }
    }

    let row_length = rows[0].len();
    let column_length = columns[0].len();

    let peripheral_trees = (row_length - 1) * 2 + (column_length - 1) * 2;

    let mut visible_trees = peripheral_trees;

    let mut trees_checked = peripheral_trees;

    for row_index in 1..rows.len() - 1 {
        let row = rows[row_index].to_owned();
        let row_chars: Vec<char> = row.chars().collect();

        for column_index in 1..row_chars.len() - 1 {
            println!("Coordinates of tree: x: {row_index}, y: {column_index}");

            let (mut a, mut b, mut c, mut d) = (false, false, false, false);
            match check_if_visible(&row, column_index, "from left") {
                true => {
                    a = true;
                }
                false => (),
            };
            match check_if_visible(&columns[column_index], row_index, "from top") {
                true => {
                    b = true;
                }
                false => (),
            };
            match check_if_visible(
                &row.chars().rev().collect(),
                row_length - column_index - 1,
                "from right",
            ) {
                true => {
                    c = true;
                }
                false => (),
            };
            match check_if_visible(
                &columns[column_index].chars().rev().collect(),
                column_length - row_index - 1,
                "from bottom",
            ) {
                true => {
                    d = true;
                }
                false => (),
            };

            if a || b || c || d {
                visible_trees += 1;

                trees_checked += 1;
            } else {
                trees_checked += 1;
            }
        }
    }

    // NOTE: Latest guess was 2153, but it was too high
    println!(
        "Total amount of trees visible: {visible_trees}, number of trees checked: {trees_checked}"
    );
}

fn check_if_visible(line: &String, index: usize, direction: &str) -> bool {
    println!("{direction}: {line}, (index: {index})");

    let line_numbers: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let (slice, _) = line_numbers.split_at(index);

    let height = line_numbers[index];

    for tree_index in 0..slice.len() - 1 {
        let tree = slice[tree_index];

        if tree >= height {
            return false;
        }
    }

    return true;
}
