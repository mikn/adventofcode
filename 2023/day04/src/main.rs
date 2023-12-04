use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let opt = &args[1];
    let file_path = &args[2];
    let file_str = read_to_string(file_path).unwrap();
    match opt.as_str() {
        "pow" => println!("Answer is: {}", winning_sum(file_str.lines())),
        "recursive" => println!("Answer is: {}", recursive_sum(file_str.lines())),
        _ => todo!(),
    }
}

fn winning_sum<'a>(lines: impl IntoIterator<Item=&'a str>) -> i32 {
    let mut result = 0;

    let numberify = |x: &str| -> HashSet<i32> {
        x.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
    };
    for line in lines {
        if let Some((_, card_winnings)) = line.split_once(':') {
            if let Some((card, winnings)) = card_winnings.split_once('|') {
                let card_nums = numberify(card);
                let winnings_nums = numberify(winnings);
                let win_len = card_nums.intersection(&winnings_nums).collect::<Vec<_>>().len();
                result += 1i32 << win_len >> 1;
            }
        }
    }
    return result;
}

fn recursive_sum<'a>(lines: impl IntoIterator<Item=&'a str>) -> usize {
    let mut result = 0;
    let mut multipliers = vec![1];

    let numberify = |x: &str| -> HashSet<usize> {
        x.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
    };
    for (i, line) in lines.into_iter().enumerate() {
        if let Some((_, card_winnings)) = line.split_once(':') {
            if let Some((card, winnings)) = card_winnings.split_once('|') {
                let card_nums = numberify(card);
                let winnings_nums = numberify(winnings);
                let win_len = card_nums.intersection(&winnings_nums).collect::<Vec<_>>().len();
                multipliers.resize_with(max(i+win_len+1, multipliers.len()), || 1);
                for j in 1..=win_len {
                    multipliers[i+j] += 1 * multipliers[i];
                }
                result += multipliers[i];
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_recursive_sum() {
        let test_string = indoc! {"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let test_data = test_string.lines();
        let res = recursive_sum(test_data);
        assert_eq!(res, 30);
    }

    #[test]
    fn test_winning_sum() {
        let test_string = indoc! {"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let test_data = test_string.lines();
        let res = winning_sum(test_data);
        assert_eq!(res, 13);
    }
}
