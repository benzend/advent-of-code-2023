mod advent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::advent;

    #[test]
    fn test_advent_day_1_part_1() {
        let data = advent::get_input(advent::AdventDay::new(1), None).unwrap();

        let mut sum: u128 = 0;

        for line in data.lines() {
            let nums = crate::first_and_last_num(line).unwrap();
            let combined = format!("{}{}", nums.0, nums.1).parse::<u128>().unwrap();
            sum += combined;
        }

        assert_eq!(sum, 55017);
    }

    #[test]
    fn test_advent_day_1_part_2() {
        let data = advent::get_input(advent::AdventDay::new(1), None).unwrap();

        let mut sum: u128 = 0;

        for line in data.lines() {
            let nums = crate::first_and_last_num_int_n_txt(line).unwrap();
            let combined = format!("{}{}", nums.0, nums.1).parse::<u128>().unwrap();
            sum += combined;
        }

        assert_eq!(sum, 53539);
    }

    #[test]
    fn test_advent_day_1_part_2_example() {
        let data = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let mut sum = 0;

        for line in data {
            let nums = crate::first_and_last_num_int_n_txt(line).unwrap();
            let combined = format!("{}{}", nums.0, nums.1).parse::<u128>().unwrap();
            sum += combined;
        }

        assert_eq!(sum, 281);
    }

    #[test]
    fn test_advent_day_1_part_2_combined_str_nums() {
        let data = vec![
            "1threetwone",      // 11
            "hhc3four88",       // 38
            "oneightfour",      // 14
            "4nineeightseven2", // 42
            "zoneight234",      // 14
        ];

        let nums = crate::first_and_last_num_int_n_txt(data[0]).unwrap();

        assert_eq!(nums, (1, 1));

        let nums = crate::first_and_last_num_int_n_txt(data[1]).unwrap();

        assert_eq!(nums, (3, 8));

        let nums = crate::first_and_last_num_int_n_txt(data[2]).unwrap();

        assert_eq!(nums, (1, 4));

        let mut sum = 0;

        for line in data {
            let nums = crate::first_and_last_num_int_n_txt(line).unwrap();
            let combined = format!("{}{}", nums.0, nums.1).parse::<u128>().unwrap();
            sum += combined;
        }

        assert_eq!(sum, 11 + 38 + 14 + 42 + 14);
    }

    #[test]
    fn test_advent_day_2_part_1() {
        let data = advent::get_input(advent::AdventDay::new(2), None).unwrap();

        let mut valid_ids = Vec::new();
        for line in data.lines() {
            let deets = line.split(":").enumerate().map(|(i, p)| {
                if i == 0 {
                    p.replace("Game ", "")
                } else {
                    p.to_string()
                }
            }).collect::<Vec<String>>();

            let id = deets[0].to_string();
            let game = deets[1].to_string();

            // only 12 red cubes, 13 green cubes, and 14 blue cubes
            let rules = crate::GameRules { red: 12, green: 13, blue: 14 };
            if crate::valid_game(crate::Game::new(&game), rules) {
                valid_ids.push(id.to_string().parse::<usize>().unwrap());
            }
        }

        let mut sum = 0;
        for id in valid_ids.iter() {
            sum += id;
        }

        assert_eq!(sum, 200);
    }
}

fn valid_game(game: Game, rules: GameRules) -> bool {
    game.max_red <= rules.red && game.max_green <= rules.green && game.max_blue <= rules.blue
}

struct GameRules {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

struct Game {
    pub max_red: usize,
    pub max_green: usize,
    pub max_blue: usize
}

impl Game {
    pub fn new(s: &str) -> Self {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let sets = s.split(";");
        for set in sets {
            let colors = set.split(",");

            for color in colors {
                let items = color.trim().split(" ").collect::<Vec<&str>>();
                let (val, label) = (items[0], items[1]);
                let val = val.parse::<usize>().unwrap();

                match label {
                    "red" => {
                        if val > max_red {
                            max_red = val;
                        }
                    }
                    "green" => {
                        if val > max_green {
                            max_green = val;
                        }
                    }
                    "blue" => {
                        if val > max_blue {
                            max_blue = val;
                        }
                    }
                    v => panic!("what kind of color is this?? ({})", v)
                }
            }
        }
        Game {
            max_red,
            max_green,
            max_blue
        }
    }
}

fn first_and_last_num(s: &str) -> Option<(u128, u128)> {
    let mut first_num = None;
    let mut last_num = None;

    for ch in s.chars() {
        if ch.is_digit(10) {
            if first_num.is_none() {
                first_num = Some(ch);
            }

            last_num = Some(ch);
        }
    }

    if first_num.is_none() || last_num.is_none() {
        None
    } else {
        Some((
            first_num.unwrap().to_digit(10).unwrap().into(),
            last_num.unwrap().to_digit(10).unwrap().into(),
        ))
    }
}

fn first_and_last_num_int_n_txt(s: &str) -> Option<(u128, u128)> {
    let first = find_first_num_int_or_txt(s);
    let last = find_last_num_int_or_txt(s);

    if first.is_none() || last.is_none() {
        None
    } else {
        Some((
            first.unwrap().to_string().parse::<u128>().unwrap().into(),
            last.unwrap().to_string().parse::<u128>().unwrap().into(),
        ))
    }
}

fn str_contains_num(s: &str) -> bool {
    s.contains("one")
        || s.contains("two")
        || s.contains("three")
        || s.contains("four")
        || s.contains("five")
        || s.contains("six")
        || s.contains("seven")
        || s.contains("eight")
        || s.contains("nine")
}

fn str_extract_num(s: &str) -> u128 {
    if s.contains("one") {
        1
    } else if s.contains("two") {
        2
    } else if s.contains("three") {
        3
    } else if s.contains("four") {
        4
    } else if s.contains("five") {
        5
    } else if s.contains("six") {
        6
    } else if s.contains("seven") {
        7
    } else if s.contains("eight") {
        8
    } else if s.contains("nine") {
        9
    } else {
        panic!("failed to find digit: {}", s);
    }
}

fn find_first_num_int_or_txt(s: &str) -> Option<String> {
    let mut word = "".to_string();
    for ch in s.chars() {
        if ch.is_digit(10) {
            return Some(ch.to_string());
        }

        word.push(ch);

        if str_contains_num(word.as_str()) {
            return Some(str_extract_num(word.as_str()).to_string());
        }
    }
    None
}

fn find_last_num_int_or_txt(s: &str) -> Option<String> {
    let mut word = "".to_string();
    for ch in s.chars().rev() {
        if ch.is_digit(10) {
            return Some(ch.to_string());
        }
        word = format!("{}{}", ch, word);

        if str_contains_num(word.as_str()) {
            return Some(str_extract_num(word.as_str()).to_string());
        }
    }
    None
}
