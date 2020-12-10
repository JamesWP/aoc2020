use std::{collections::HashMap, collections::HashSet, fs};

fn how_many_ways(n: i64, set: &HashSet<i64>, cache: &mut HashMap<i64, i64>) -> i64 {
    if n == 0 {
        return 1;
    }

    if let Some(n) = cache.get(&n) {
        return *n;
    }

    if !set.contains(&n) {
        return 0;
    }

    let n1 = how_many_ways(n - 1, set, cache);
    let n2 = how_many_ways(n - 2, set, cache);
    let n3 = how_many_ways(n - 3, set, cache);

    cache.insert(n, n1 + n2 + n3);
    return n1 + n2 + n3;
}

#[test]
fn test() {
    let contents = fs::read_to_string("day10.txt").expect("Something went wrong reading the file");

    let mut adapters: Vec<i64> = contents
        .split("\n")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    adapters.sort();

    let diffs = adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .map(|(a, b)| b - a);

    let mut counter = HashMap::new();

    diffs.for_each(|diff| {
        let count = counter.entry(diff).or_insert(1);

        *count += 1;
    });

    println!("counts: {:?}", counter);

    println!(
        "prod: {}",
        counter.get(&3).unwrap() * counter.get(&1).unwrap()
    );

    let max = adapters.iter().max().unwrap() + 3;
    let set: HashSet<i64> = adapters.iter().cloned().collect();

    println!("max: {}", max);

    let mut cache = HashMap::new();

    println!(
        "combs: {}",
        how_many_ways(max - 1, &set,&mut cache) + how_many_ways(max - 2, &set,&mut cache) + how_many_ways(max - 3, &set,&mut cache)
    );
}
