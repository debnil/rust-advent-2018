use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let mut seen = HashSet::new();
    seen.insert(0);

    let f = File::open("1-data.txt").unwrap();
    let file = BufReader::new(&f);
    let mut v = vec![];
    let mut final_freq = 0;
    for line in file.lines() {
        let change: i32 = line.unwrap().to_string().parse::<i32>().unwrap();
        final_freq += change;
        v.push(change);
    }
    println!("The final frequency is: {}.", final_freq);
    let mut freq = 0;
    'outer: loop{
        for &change in &v {
            freq += change;
            if seen.contains(&freq) {
                println!("Found duplicate frequency: {}", freq);
                break 'outer;
            }
            seen.insert(freq);
        }
    }
}