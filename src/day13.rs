use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fs};

#[test]
fn test_things() {
    let contents = fs::read_to_string("day13.txt").expect("Something went wrong reading the file");

    let mut lines = contents.split("\n");
    let depart_after: u64 = lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<Option<u64>> = lines
        .next()
        .unwrap()
        .split(",")
        .map(str::parse)
        .map(Result::ok)
        .collect();

    let mods: (u64, u64) = bus_ids
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

#[test]
fn part2() {
    let input = "17,x,x,x,x,x,x,41,x,x,x,37,x,x,x,x,x,367,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19,x,x,x,23,x,x,x,x,x,29,x,613,x,x,x,x,x,x,x,x,x,x,x,x,13";
    //let input = "7,13,x,x,59,x,31,19";
    //let input = "17,x,13,19";
    //let input = "1789,37,47,1889";

    let depart_after: Vec<(u64, u64)> = input
        .split(",")
        .map(str::parse::<u64>)
        .zip(0..)
        .filter(|(bus, _offset)| bus.is_ok())
        .map(|(bus, offset)| (bus.unwrap(), offset))
        .map(|(bus, offset)| (bus, (bus - (offset % bus)) % bus))
        .collect();

    println!("busses: {:?}", depart_after);

    let mut divisor = 1;
    let mut remainder = 0;

    for (bus, bus_offset) in depart_after {
        let offset = (0..)
            .map(|i| i * divisor + remainder)
            .skip_while(|val| val % bus != bus_offset)
            .next()
            .unwrap();

        divisor *= bus;
        remainder = offset;

        println!("busses: {:10}x+{}", divisor, remainder);
    }

    println!("busses: {:10}x+{}", divisor, remainder);

    let series = (0..).map(|v| v * divisor + remainder);

    println!("b1: {}", series.clone().next().unwrap());
}
