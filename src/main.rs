use std::{collections::HashSet, fs};
use itertools::Itertools;




fn main() {
    let contents = fs::read_to_string("day1.txt")
        .expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();

    let compliment_set: HashSet<i32> = numbers.iter().cloned().collect();

    let twentytwenty: HashSet<i32> = compliment_set.iter().cloned().filter(|val|compliment_set.contains(&(2020-val))).collect();

    println!("twentytwenty: {:?}", twentytwenty);

    println!("result: {:?}", twentytwenty.iter().fold(1, |acc, val|acc * val));

    let twentytwentytwenty: HashSet<i32> = compliment_set
        .iter()
        .cloned()
        .cartesian_product(compliment_set.iter().cloned())
        .filter(|(a,b)| compliment_set.contains(&(2020 - (a + b))))
        .map(|(a,_b)| a)
        .collect();

    println!("twentytwentytwenty: {:?}", twentytwentytwenty);

    println!("result: {:?}", twentytwentytwenty.iter().fold(1, |acc, val|acc * val));
}
