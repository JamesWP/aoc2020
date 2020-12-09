use std::{collections::HashSet, fs};

#[test]
fn test() {
    let contents = fs::read_to_string("day9.txt").expect("Something went wrong reading the file");
    let num = 25;

    let ciphertext: Vec<i64> = contents
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let first_not_summing = ciphertext
        .iter()
        .cloned()
        .enumerate()
        .skip(num)
        .map(|(idx, val)| (idx, val, &ciphertext[idx - num..idx]))
        .filter(|(_idx, val, nums)| {
            let set: HashSet<i64> = nums.iter().cloned().collect();
            set.iter()
                .cloned()
                .filter(|n| set.contains(&(val - n)))
                .next()
                .is_none()
        })
        .map(|(_idx, val, _nums)| val)
        .next()
        .unwrap();

    println!("bad: {}", first_not_summing);

    let solution = ciphertext
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, val)| {
            //println!("start");
            let mut sum = val;
            for (len, i) in (0..idx).rev().enumerate() {
                let v = ciphertext.get(i).unwrap();
                //println!("s: {}", v);
                sum += v;
                if sum == first_not_summing {
                    //println!("sum");
                    return (true, len + 1, idx);
                }
                if sum > first_not_summing {
                    break;
                }
            }
            //println!("end");
            return (false, 0, idx);
        })
        .filter(|(matches, _len, _idx)| *matches)
        .map(|(_matches, len, idx)| &ciphertext[idx - len..=idx])
        .next();

    // not 3607989
    println!(
        "sol: {:?}, min: {:?}, max: {:?}",
        solution,
        solution.unwrap().iter().min(),
        solution.unwrap().iter().max()
    );
}
