use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn calculate(reader: &mut dyn BufRead) -> isize {
    let mut counts: Vec<usize> = Vec::new();
    let mut line_count = 0;

    for line in reader.lines() {
        let row: Vec<char> = line.unwrap().trim().chars().collect();
        counts.resize(row.len(), 0);
        for (i, c) in row.iter().enumerate() {
            if c == &'1' {
                counts[i] += 1;
            }
        }
        line_count += 1;
    }

    let max_val = 2_isize.pow(counts.len().try_into().unwrap())-1;
    let mut gamma_str = String::new();
    for count in counts {
        gamma_str.push_str(if line_count/2 < count { "1" } else { "0" })
    }

    let gamma = isize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = max_val ^ gamma;
    println!("gamma binary: {0:b} decimal: {0}", gamma);
    println!("epsilon binary: {0:b} decimal: {0}", epsilon);
    return gamma * epsilon;

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {}", filename);

    let f = File::open(filename).unwrap();
    let mut file = BufReader::new(&f);

    println!("The product of gamma and epsilon is: {}", calculate(&mut file));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate() {
        let mut test_data = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n\
                         11100\n10000\n11001\n00010\n01010".as_bytes();
        let test_result = 198;
        assert_eq!(calculate(&mut test_data), test_result);
    }
}
