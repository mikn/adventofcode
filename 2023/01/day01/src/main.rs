use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn conv_digit(number_map: &HashMap<String, String>, input: String) -> String {
    match number_map.get(&input) {
        Some(i) => {
            return i.to_string();
        }
        None => {
            return input.to_string();
        }
    }
}

fn find_digits_text<'a>(lines: impl IntoIterator<Item=&'a str>) -> i32 {
    let mut number_map = HashMap::new();
    let number_lines = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eigthy", "ninety", "hundred"];
    for number in 0..20 {
        number_map.insert(number_lines[number].to_string(), format!("{}", number).to_string());
    }
    number_map.insert(number_lines[21].to_string(), "30".to_string());
    number_map.insert(number_lines[22].to_string(), "40".to_string());
    number_map.insert(number_lines[23].to_string(), "50".to_string());
    number_map.insert(number_lines[24].to_string(), "60".to_string());
    number_map.insert(number_lines[25].to_string(), "70".to_string());
    number_map.insert(number_lines[26].to_string(), "80".to_string());
    number_map.insert(number_lines[27].to_string(), "90".to_string());
    number_map.insert(number_lines[28].to_string(), "100".to_string());

    let regex_digits = number_lines.join("|");
    let forward_regex = format!(r"^.*?(\d|{})", regex_digits);
    let reverse_regex = format!(r".*(\d|{}).*?$", regex_digits);
    let forward = Regex::new(forward_regex.as_str()).unwrap();
    let reverse = Regex::new(reverse_regex.as_str()).unwrap();

    let lines_iter = lines.into_iter();
    let mut res: i32 = 0;
    for line in lines_iter {
        let forward_match = forward.captures(line).unwrap().get(1).unwrap().as_str();
        let reverse_match = reverse.captures(line).unwrap().get(1).unwrap().as_str();
        let forward_string = conv_digit(&number_map, forward_match.to_string());
        let reverse_string = conv_digit(&number_map, reverse_match.to_string());
        let int_str = format!("{forward_string}{reverse_string}");
        let int_val = int_str.parse::<i32>().unwrap();
        res += int_val;
    }
    return res
}

fn find_digits<'a>(lines: impl IntoIterator<Item=&'a str>) -> i32 {
    let forward = Regex::new(r"^.*?(\d)").unwrap();
    let reverse = Regex::new(r".*(\d).*?$").unwrap();

    let lines_iter = lines.into_iter();
    let mut res: i32 = 0;
    for line in lines_iter {
        let forward_match = forward.captures(line).unwrap().get(1).unwrap().as_str();
        let reverse_match = reverse.captures(line).unwrap().get(1).unwrap().as_str();
        let int_str = format!("{forward_match}{reverse_match}");
        let int_val = int_str.parse::<i32>().unwrap();
        res += int_val;
    }
    return res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opt = &args[1];
    let file_path = &args[2];
    let file_str = read_to_string(file_path).unwrap();
    match opt.as_str() {
        "numbers" => println!("{}", find_digits(file_str.lines())),
        "text" => println!("{}", find_digits_text(file_str.lines())),
        _ => println!("Options are 'numbers' and 'text'"),
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;
    #[test]
    fn find_digits_sample_data() {
        let test_string = indoc! {"1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        let test_data = test_string.lines();
        let res = find_digits(test_data);
        assert_eq!(res, 142);
    }

    #[test]
    fn find_digits_text_sample_data() {
        let test_string = indoc! {"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        let test_data = test_string.lines();
        let res = find_digits_text(test_data);
        assert_eq!(res, 281);
    }
}
