use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {}", filename);

    let f = File::open(filename).unwrap();

    let file = BufReader::new(&f);

    let mut depth = 0;
    let mut aim = 0;
    let mut x = 0;

    for line in file.lines() {
        let l = line.unwrap();
        let vals = l.trim().split_whitespace().collect::<Vec<&str>>();
        let val = vals[1].parse::<i64>().unwrap();
        match vals[0] {
            "forward" => {
                x += val;
                depth += aim * val;
            },
            "up" => aim -= val,
            "down" => aim += val,
            _ => panic!("Command {} not supported!", vals[0]),
        }
    }
    println!("The product of depth and horizontal position is: {}!", depth*x);
}
