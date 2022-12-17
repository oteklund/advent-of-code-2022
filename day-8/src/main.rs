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

    let mut best_view = 0;

    for row_index in 1..rows.len() - 1 {
        let row = rows[row_index].to_owned();
        let row_chars: Vec<char> = row.chars().collect();

        for column_index in 1..row_chars.len() - 1 {
            let (mut a, mut b, mut c, mut d) = (false, false, false, false);
            let (row_from_left, index_from_left) = (&row, column_index);
            let (column_from_top, index_from_top) = (&columns[column_index], row_index);
            let (row_from_right, index_from_right): (&String, usize) =
                (&row.chars().rev().collect(), row_length - column_index - 1);
            let (column_from_bottom, index_from_bottom): (&String, usize) = (
                &columns[column_index].chars().rev().collect(),
                column_length - row_index - 1,
            );

            match check_if_visible(&row_from_left, index_from_left, "from left") {
                true => {
                    a = true;
                }
                false => (),
            };
            match check_if_visible(&column_from_top, index_from_top, "from top") {
                true => {
                    b = true;
                }
                false => (),
            };
            match check_if_visible(&row_from_right, index_from_right, "from right") {
                true => {
                    c = true;
                }
                false => (),
            };
            match check_if_visible(&column_from_bottom, index_from_bottom, "from bottom") {
                true => {
                    d = true;
                }
                false => (),
            };

            if a || b || c || d {
                println!("Coordinates of tree: x: {row_index}, y: {column_index}");

                visible_trees += 1;

                println!("calculating view score for tree at {row_index}, {column_index}");
                let view_score = calculate_view_score(
                    (&row_from_left, index_from_left),
                    (&column_from_top, index_from_top),
                );

                if view_score > best_view {
                    best_view = view_score;
                }

                trees_checked += 1;
            } else {
                trees_checked += 1;
            }
        }
    }

    println!(
        "Total amount of trees visible: {visible_trees}, number of trees checked: {trees_checked}, best view: {best_view}"
    );
}

fn check_if_visible(line: &String, index: usize, _direction: &str) -> bool {
    let line_numbers: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let (slice, _) = line_numbers.split_at(index);

    let height = line_numbers[index];

    for tree_index in 0..slice.len() {
        let tree = slice[tree_index];

        if tree >= height {
            return false;
        }
    }

    return true;
}

// test input:
// 30373
// 25512
// 65332
// 33549
// 35390

fn calculate_view_score(
    (row, row_index): (&String, usize),
    (col, col_index): (&String, usize),
) -> u32 {
    println!("row: {row}, col: {col}");

    let (
        mut view_distance_right,
        mut view_distance_down,
        mut view_distance_left,
        mut view_distance_up,
    ) = (0, 0, 0, 0);

    let row_split = &row
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let col_split = &col
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let height = row_split[row_index];

    for i in (row_index + 1)..row_split.len() {
        view_distance_right += 1;

        if row_split[i] >= height {
            break;
        }
    }
    for i in (0..(row_index)).rev() {
        view_distance_left += 1;

        if row_split[i] >= height {
            break;
        }
    }
    for i in (col_index + 1)..col_split.len() {
        view_distance_down += 1;

        if (col_split[i]) >= height {
            break;
        }
    }
    for i in (0..(col_index)).rev() {
        view_distance_up += 1;

        if (col_split[i]) >= height {
            break;
        }
    }

    println!(
        "Tree height: {}, view distances: right - {view_distance_right}, down - {view_distance_down}, left - {view_distance_left}, up - {view_distance_up}",
        row_split[row_index]
    );

    let result = view_distance_right * view_distance_down * view_distance_left * view_distance_up;

    return result;
}
