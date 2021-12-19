use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {}", filename);

    let f = File::open(filename).unwrap();

    let mut file = BufReader::new(&f);

    let mut first_line = String::new();
    file.read_line(&mut first_line).unwrap();

    let mut prev_val = match first_line.trim().parse::<i64>() {
        Ok(prev_val) => prev_val,
        Err(e) => {
            println!("line {} produced error {}", first_line, e);
            return Err(e);
        },
    };
    let mut inc_count = 0;
    for line in file.lines() {
        let val = line.unwrap().trim().parse::<i64>().unwrap();
        if val > prev_val {
            inc_count += 1;
        }
        prev_val = val;
    }
    println!("The value increased {} times from the previous value!", inc_count);
    Ok(())
}
