use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fs};

#[test]
fn testing() {
    let contents =
        fs::read_to_string("day15.txt").expect("Something went wrong reading the file");

    let nums: Vec<i32> = contents
        .split(",")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect();

    let mut mem: HashMap<i32, i32> = nums[0..nums.len()-1].iter().cloned().zip(1..).collect();

    println!("mem: {:?}", mem);

    let starting = contents.split(",").count();
    let starting = i32::try_from(starting).unwrap();

    let res = (starting..).take(29999994).fold(nums.last().cloned().unwrap(), |last, iter| {
        let spoken = mem.get(&last).cloned().map(|v| iter-v).unwrap_or(0);
        mem.insert(last, iter);
        spoken
    });

    println!("last: {:?}", res);
}
