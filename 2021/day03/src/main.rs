use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &[u8] = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n\
                               11100\n10000\n11001\n00010\n01010".as_bytes();

    #[test]
    fn test_calc_power() {
        let mut test_data = TEST_DATA;
        let test_result = 198;
        assert_eq!(calc_power(&mut test_data), test_result);
    }

    #[test]
    fn test_calc_life() {
        let mut test_data = TEST_DATA;
        let test_result = 230;
        assert_eq!(calc_life(&mut test_data), test_result);
    }
}

fn calc_power(reader: &mut dyn BufRead) -> isize {
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

fn filter_lines(lines: Vec<isize>, max_val: isize) -> (Vec<isize>, Vec<isize>) {
    let mut ones: Vec<isize> = Vec::new();
    let mut zeroes: Vec<isize> = Vec::new();
    for line in lines {
        if line & max_val == max_val {
            ones.push(line);
        } else {
            zeroes.push(line);
        }
    }
    if ones.len() >= zeroes.len() {
        return (ones, zeroes);
    } else {
        return (zeroes, ones);
    }
}

fn calc_life(reader: &mut dyn BufRead) -> isize {
    let mut lines: Vec<isize> = Vec::new();

    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    first_line = first_line.trim().to_string();
    lines.push(isize::from_str_radix(&first_line, 2).unwrap());

    let bit_count: u32 = first_line.len().try_into().unwrap();

    for line in reader.lines() {
        let l = line.unwrap();
        lines.push(isize::from_str_radix(&l.trim(), 2).unwrap());
    }

    let (mut most_common, mut least_common) = filter_lines(lines, max_val);

    let mut max_val = 2_isize.pow(bit_count-1);
    while most_common.len() > 1 {
        max_val = max_val >> 1;
        (most_common, _) = filter_lines(most_common, max_val);
    }

    max_val = 2_isize.pow(bit_count-1);
    while least_common.len() > 1 {
        max_val = max_val >> 1;
        (_, least_common) = filter_lines(least_common, max_val);
    }

    let most = most_common.pop().unwrap();
    let least = least_common.pop().unwrap();
    println!("most_common binary: {0:b} decimal: {0}", most);
    println!("least_common binary: {0:b} decimal: {0}", least);
    return most * least;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {}", filename);

    let f = File::open(filename).unwrap();
    let mut file = BufReader::new(&f);

    println!("The product of gamma and epsilon is: {}", calc_power(file.by_ref()));
    file.seek(SeekFrom::Start(0)).unwrap();
    println!("The product of oxygen and co2 is: {}", calc_life(file.by_ref()));
}

