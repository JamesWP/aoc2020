use std::fs;

fn counter(rows_per_step: usize, cols_per_step: usize, rows: &Vec<&str>) -> i64 {
    let modulo = rows[0].len();

    (0..)
        .map(|index| (index * rows_per_step, (index * cols_per_step) % modulo))
        .take(rows.len() / rows_per_step)
        .map(|(row, col)| rows[row].chars().nth(col).unwrap())
        .map(|cell| if cell == '#' { 1 } else { 0 })
        .sum()
}

#[test]
fn day3() {
    let contents = fs::read_to_string("day3.txt").expect("Something went wrong reading the file");

    let rows: Vec<&str> = contents.split("\n").collect();

    println!("trees hit: {}", counter(1, 3, &rows));
}

#[test]
fn day3part2() {
    let contents = fs::read_to_string("day3.txt").expect("Something went wrong reading the file");

    let rows: Vec<&str> = contents.split("\n").collect();

    let product = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .cloned()
        .map(|(r, c)| counter(r, c, &rows))
        .fold(1, |acc, val| acc * val);

    println!("product: {}", product);
}
