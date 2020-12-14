use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fs};

#[test]
fn test_things() {
    let contents =
        fs::read_to_string("day13.txt").expect("Something went wrong reading the file");

    let mut lines = contents.split("\n");
    let depart_after: i32 = lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<Option<i32>> = lines
        .next()
        .unwrap()
        .split(",")
        .map(str::parse)
        .map(Result::ok)
        .collect();

    let mods: (i32, i32) = bus_ids
        .iter()
        .cloned()
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(|bus_id| (bus_id - (depart_after % bus_id), bus_id))
        .min()
        .unwrap();

    println!("mods: {:?}, prod: {}", mods, mods.0 * mods.1);

    println!("after: {} ids: {:?}", depart_after, bus_ids);
}
