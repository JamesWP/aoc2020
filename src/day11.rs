use std::mem;
use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fs};

fn print_seats(seats: &Vec<char>, rows: i32, cols: i32) {
    for i in 0..rows {
        let row_start: usize = (i * cols).try_into().unwrap();
        let row_end: usize = (i * cols + cols).try_into().unwrap();

        let row_range = row_start..row_end;
        let row: String = seats[row_range].iter().collect();
        println!("{}", row);
    }
}

fn neighbours(seat_pos: i32, grid: &Vec<char>, rows: i32, cols: i32) -> Vec<i32> {
    let seat_row = seat_pos / cols;
    let seat_col = seat_pos % cols;
    let directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    directions
        .iter()
        .map(|(rd, cd)| (1..)
            .map(|i|(seat_row + rd*i, seat_col + cd*i))
            .take_while(|(row, col)| (0..rows).contains(row) && (0..cols).contains(col))
            .skip_while(|(row, col)| {
                let idx:usize = usize::try_from(row*cols + col).unwrap();
                let cell = grid.get(idx).cloned().unwrap_or('.');
                cell != '#' && cell != 'L'
            })
            .next()
        )
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(|(row, col)| row * cols + col)
        .collect()
}

fn seat_rule(seat_pos: i32, neighbours: &Vec<i32>, cur: &Vec<char>, _rows: i32, _cols: i32) -> char {
    let seat = cur[usize::try_from(seat_pos).unwrap()];
    let occupied = neighbours
        .iter()
        .cloned()
        .map(|n| usize::try_from(n).unwrap())
        .map(|n| cur.get(n))
        .filter(|s| match *s {
            Some(c) if *c == '#' => true,
            _ => false,
        })
        .count();
    match (seat, occupied) {
        ('L', 0) => '#',
        ('#', count) if count >= 5 => 'L',
        _ => seat,
    }
}

fn iterate(
    prev: &Vec<char>,
    next: &mut Vec<char>,
    nmap: &HashMap<i32, Vec<i32>>,
    rows: i32,
    cols: i32,
) {
    next.iter_mut().zip(0..).for_each(|(seat, pos)| {
        let neighbours = nmap.get(&pos).unwrap();
        *seat = seat_rule(pos, neighbours, &prev, rows, cols);
    });
}

#[test]
fn test() {
    let contents = fs::read_to_string("day11.txt").expect("Something went wrong reading the file");

    let rows: i32 = contents.split("\n").count().try_into().unwrap();
    let cols: i32 = contents
        .split("\n")
        .next()
        .unwrap()
        .len()
        .try_into()
        .unwrap();

    println!("rows: {}, cols: {}", rows, cols);
    let start_grid: Vec<char> = contents.chars().filter(|c| *c != '\n').collect();

    let (mut a, mut b) = (
        start_grid.clone(),
        start_grid.iter().map(|_a| ' ').collect(),
    );

    let nmap: HashMap<i32, Vec<i32>> = (0..(rows * cols))
        .map(|p| (p, neighbours(p, &start_grid, rows, cols)))
        .collect();

    loop {
        //print_seats(&a, rows, cols);
        //println!("n10:{:?}", nmap.get(&10));
        //println!("10'sns:{:?}", nmap.get(&10).unwrap().iter().map(|i|a.get(usize::try_from(*i).unwrap()).unwrap()));
        iterate(&a, &mut b, &nmap, rows, cols);

        mem::swap(&mut a, &mut b);

        let occupied = a.iter().cloned().filter(|s| *s == '#').count();
        println!("ocu: {}", occupied);

        if a == b {
            break;
        }
    }

    print_seats(&a, rows, cols);

    let occupied = a.iter().cloned().filter(|s| *s == '#').count();

    println!("ocu: {}", occupied);
    // 2568 too high
    // 2483 correct
}
