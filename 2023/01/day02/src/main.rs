use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn sum_game(input_str: String) -> i32 {
    let mut sum_map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for round in input_str.split(";") {
        for color_play in round.split(",") {
            if let Some((i, color)) = color_play.trim().split_once(' ') {
                let i_int = i.parse::<i32>().unwrap();
                if sum_map.get(color).unwrap() < &i_int {
                    sum_map.insert(color, i_int);
                }
            }
        }
    }
    return sum_map.values().product();
}

fn min_games<'a>(lines: impl IntoIterator<Item=&'a str>) -> i32 {
    let mut result = 0;
    for line in lines {
        if let Some((_, game)) = line.split_once(':') {
            result += sum_game(game.to_string());
        }
    }
    return result;
}

fn possible_game(input_str: String, color_limits: &HashMap<&str, i32>) -> bool {
    for round in input_str.split(";") {
        for color_play in round.split(",") {
            if let Some((i, color)) = color_play.trim().split_once(' ') {
                let i_int = i.parse::<i32>().unwrap();
                if color_limits.get(color).unwrap() < &i_int {
                    return false;
                }
            }
        }
    }
    return true;
}

fn eval_games<'a>(lines: impl IntoIterator<Item=&'a str>) -> i32 {
    let mut result = 0;
    let color_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for line in lines {
        if let Some((game_no, game)) = line.split_once(':') {
            if let Some((_, game_int)) = game_no.split_once(' ') {
                if possible_game(game.to_string(), &color_limits) {
                    result += game_int.parse::<i32>().unwrap();
                }
            }
        }
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opt = &args[1];
    let file_path = &args[2];
    let file_str = read_to_string(file_path).unwrap();
    match opt.as_str() {
        "possible" => println!("{}", eval_games(file_str.lines())),
        "min" => println!("{}", min_games(file_str.lines())),
        _ => println!("Options are 'possible' and 'min'"),
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_eval_games() {
        let test_string = indoc! {"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let test_data = test_string.lines();
        let res = eval_games(test_data);
        assert_eq!(res, 8);
    }

    #[test]
    fn test_min_games() {
        let test_string = indoc! {"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let test_data = test_string.lines();
        let res = min_games(test_data);
        assert_eq!(res, 2286);
    }
}
