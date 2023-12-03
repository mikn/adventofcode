use regex::Regex;
use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

#[derive(Clone)]
#[derive(Debug)]
struct PartNum {
    pos: usize,
    len: usize,
    num: i32,
}

struct LineParts {
    parts: Vec<usize>,
    part_nums: Vec<PartNum>,
}

struct GearCandidate {
    pos: usize,
    part_nums: Vec<PartNum>,
}

struct LineGears {
    gears: Vec<GearCandidate>,
    part_nums: Vec<PartNum>,
}

fn gear_finder<'a>(lines: impl IntoIterator<Item=&'a str>) -> i32 {
    let mut result = 0;
    let mut line_gears_buffer = VecDeque::from([
        LineGears{gears: vec![], part_nums: vec![]},
        LineGears{gears: vec![], part_nums: vec![]},
    ]);
    let gear_regex = Regex::new(r"\*").unwrap();
    let part_num_regex = Regex::new(r"\d+").unwrap();
    for line in lines {
        let mut line_gears = line_gears_buffer.pop_front().unwrap();
        let mut last_line_gears = line_gears_buffer.pop_front().unwrap();
        line_gears.part_nums.clear();
        line_gears.gears.clear();

        let mut gear_pos: Vec<GearCandidate> = gear_regex.find_iter(line).map(|m| GearCandidate{pos: m.start(), part_nums: vec![]}).collect();
        let part_nums: Vec<PartNum> = part_num_regex.find_iter(line).map(|m| PartNum{pos: m.start(), len: m.len(), num: m.as_str().parse::<i32>().unwrap()}).collect();

        let gather_part_nums = |x: Vec<PartNum>, part_p: usize| x.iter().filter(move |p| (p.pos <= part_p+1 && p.pos+p.len >= part_p)).map(|p| p.clone()).collect::<Vec<_>>();

        for gear_c in &mut gear_pos {
            gear_c.part_nums.extend(gather_part_nums(part_nums.clone(), gear_c.pos));
            gear_c.part_nums.extend(gather_part_nums(last_line_gears.part_nums.clone(), gear_c.pos));
        }

        for gear_c in &mut last_line_gears.gears {
            // TODO only by this point we would know if a gear qualifies or not
            gear_c.part_nums.extend(gather_part_nums(part_nums.clone(), gear_c.pos));
            if gear_c.part_nums.len() == 2 {
                result += gear_c.part_nums.iter().map(|p| p.num).product::<i32>();
            }
        }

        line_gears.gears.extend(gear_pos);
        line_gears.part_nums.extend(part_nums);

        line_gears_buffer.push_back(last_line_gears);
        line_gears_buffer.push_back(line_gears);
    }
    return result;
}

fn part_finder<'a>(lines: impl IntoIterator<Item=&'a str>) -> i32 {
    let mut result = 0;
    let mut line_parts_buffer = VecDeque::from([
        LineParts{parts: vec![], part_nums: vec![]},
        LineParts{parts: vec![], part_nums: vec![]},
    ]);
    let part_regex = Regex::new(r"[^\d.]").unwrap();
    let part_num_regex = Regex::new(r"\d+").unwrap();
    for line in lines {
        let mut line_parts = line_parts_buffer.pop_front().unwrap();
        let last_line_parts = line_parts_buffer.pop_front().unwrap();
        line_parts.part_nums.clear();
        line_parts.parts.clear();

        let part_pos: Vec<usize> = part_regex.find_iter(line).map(|m| m.start()).collect();
        let part_nums: Vec<PartNum> = part_num_regex.find_iter(line).map(|m| PartNum{pos: m.start(), len: m.len(), num: m.as_str().parse::<i32>().unwrap()}).collect();

        let sum_part_nums = |x: &Vec<PartNum>, part_p: usize| x.iter().filter(|p| (p.pos <= part_p+1 && p.pos+p.len >= part_p)).map(|p| p.num).collect::<Vec<i32>>().iter().sum::<i32>();

        for part_p in &part_pos {
            result += sum_part_nums(&part_nums, *part_p);
            result += sum_part_nums(&last_line_parts.part_nums, *part_p);
        }

        for part_p in &last_line_parts.parts {
            result += sum_part_nums(&part_nums, *part_p);
        }

        line_parts.parts.extend(part_pos);
        line_parts.part_nums.extend(part_nums);

        line_parts_buffer.push_back(last_line_parts);
        line_parts_buffer.push_back(line_parts);
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opt = &args[1];
    let file_path = &args[2];
    let file_str = read_to_string(file_path).unwrap();
    match opt.as_str() {
        "adjacent" => println!("{}", part_finder(file_str.lines())),
        "gears" => println!("{}", gear_finder(file_str.lines())),
        _ => println!("Options are 'adjacent' and 'min'"),
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_gear_finder() {
        let test_string = indoc! {"467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let test_data = test_string.lines();
        let res = gear_finder(test_data);
        println!("{}", res-467835);
        assert_eq!(res, 467835);
    }

    #[test]
    fn test_part_finder() {
        let test_string = indoc! {"467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let test_data = test_string.lines();
        let res = part_finder(test_data);
        println!("{}", res-4361);
        assert_eq!(res, 4361);
    }
}
