use std::{
    cmp::max, cmp::min, collections::HashMap, collections::HashSet, convert::TryFrom,
    convert::TryInto, fs, mem, ops::RangeInclusive,
};

use itertools::Itertools;

type World = HashSet<(i32, i32, i32, i32)>;

fn extend(range: RangeInclusive<i32>, val: i32) -> RangeInclusive<i32> {
    let start = min(*range.start(), val);
    let end = max(*range.end(), val);
    start..=end
}

fn bounds(
    m: &World,
) -> (
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
) {
    m.iter().fold(
        (0..=0, 0..=0, 0..=0, 0..=0),
        |(x, y, z, w), (x_v, y_v, z_v, w_v)| {
            (
                extend(x, *x_v),
                extend(y, *y_v),
                extend(z, *z_v),
                extend(w, *w_v),
            )
        },
    )
}

fn print(world: &World) {
    let (xs, ys, zs, ws) = bounds(&world);

    for w in ws.clone() {
        for z in zs.clone() {
            println!("z={}, w={}", z, w);
            for y in ys.clone() {
                println!(
                    "{}",
                    xs.clone()
                        .map(|x| if world.contains(&(x, y, z, w)) { '#' } else { '.' })
                        .collect::<String>()
                );
            }
            println!("");
        }
    }
}

fn advance(source: &World, mut dest: World) -> World {
    dest.clear();
    let (xs, ys, zs, ws) = bounds(source);
    let xs = (xs.start() - 1)..=(xs.end() + 1);
    let ys = (ys.start() - 1)..=(ys.end() + 1);
    let zs = (zs.start() - 1)..=(zs.end() + 1);
    let ws = (ws.start() - 1)..=(ws.end() + 1);
    for w in ws.clone() {
        for z in zs.clone() {
            for y in ys.clone() {
                for x in xs.clone() {
                    let active = source.contains(&(x, y, z, w));
                    let mut nc = 0;
                    for wo in (w - 1)..=(w + 1) {
                        for zo in (z - 1)..=(z + 1) {
                            for yo in (y - 1)..=(y + 1) {
                                for xo in (x - 1)..=(x + 1) {
                                    nc += if source.contains(&(xo, yo, zo, wo)) {
                                        1
                                    } else {
                                        0
                                    }
                                }
                            }
                        }
                    }
                    if active {
                        nc -= 1
                    }

                    if active && (2 == nc || nc == 3) {
                        dest.insert((x, y, z, w));
                    } else if !active && nc == 3 {
                        dest.insert((x, y, z, w));
                    }
                }
            }
        }
    }
    dest
}

#[test]
fn read_input() {
    let contents = fs::read_to_string("day17.txt").expect("Something went wrong reading the file");

    let starting: World = contents
        .split("\n")
        .zip(0..)
        .map(|(row, row_idx)| {
            row.chars()
                .zip(0..)
                .filter(|(c, _)| *c == '#')
                .map(|(_, col_idx)| (col_idx, row_idx, 0, 0))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    println!("starting: {:?}", starting);
    println!("bounds: {:?}", bounds(&starting));
    print(&starting);

    let mut prev = starting.clone();
    let mut next = HashSet::new();

    for iter in 1..=6 {
        println!("iter:{}", iter);

        next = advance(&prev, next);

        print(&next);

        mem::swap(&mut next, &mut prev);
    }

    println!("count: {}", prev.len());
}
