mod advent;
mod day_1;
mod day_2;

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
            let nums = crate::day_1::first_and_last_num(line).unwrap();
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
            let nums = crate::day_1::first_and_last_num_int_n_txt(line).unwrap();
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
            let nums = crate::day_1::first_and_last_num_int_n_txt(line).unwrap();
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

        let nums = crate::day_1::first_and_last_num_int_n_txt(data[0]).unwrap();

        assert_eq!(nums, (1, 1));

        let nums = crate::day_1::first_and_last_num_int_n_txt(data[1]).unwrap();

        assert_eq!(nums, (3, 8));

        let nums = crate::day_1::first_and_last_num_int_n_txt(data[2]).unwrap();

        assert_eq!(nums, (1, 4));

        let mut sum = 0;

        for line in data {
            let nums = crate::day_1::first_and_last_num_int_n_txt(line).unwrap();
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
            let deets = line
                .split(":")
                .enumerate()
                .map(|(i, p)| {
                    if i == 0 {
                        p.replace("Game ", "")
                    } else {
                        p.to_string()
                    }
                })
                .collect::<Vec<String>>();

            let id = deets[0].to_string();
            let game = deets[1].to_string();

            // only 12 red cubes, 13 green cubes, and 14 blue cubes
            let rules = crate::day_2::GameRules {
                red: 12,
                green: 13,
                blue: 14,
            };
            if crate::day_2::valid_game(crate::day_2::Game::new(&game), rules) {
                valid_ids.push(id.to_string().parse::<usize>().unwrap());
            }
        }

        let mut sum = 0;
        for id in valid_ids.iter() {
            sum += id;
        }

        assert_eq!(sum, 2076);
    }

    #[test]
    fn test_advent_day_2_part_2() {
        let data = advent::get_input(advent::AdventDay::new(2), None).unwrap();

        let mut sum = 0;
        for line in data.lines() {
            let deets = line
                .split(":")
                .enumerate()
                .map(|(i, p)| {
                    if i == 0 {
                        p.replace("Game ", "")
                    } else {
                        p.to_string()
                    }
                })
                .collect::<Vec<String>>();

            let game = deets[1].to_string();

            let game = crate::day_2::Game::new(&game);
            sum += game.max_red * game.max_green * game.max_blue;
        }

        assert_eq!(sum, 70950);
    }

    #[test]
    fn test_advent_day_2_part_2_example() {
        let data = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let mut sum = 0;
        for (i, line) in data.iter().enumerate() {
            let deets = line
                .split(":")
                .enumerate()
                .map(|(i, p)| {
                    if i == 0 {
                        p.replace("Game ", "")
                    } else {
                        p.to_string()
                    }
                })
                .collect::<Vec<String>>();

            let game = deets[1].to_string();

            let game = crate::day_2::Game::new(&game);

            if i == 0 {
                assert_eq!(game.max_red, 4);
                assert_eq!(game.max_green, 2);
                assert_eq!(game.max_blue, 6);
            }

            sum += game.max_red * game.max_green * game.max_blue;
        }

        assert_eq!(sum, 2286);
    }
}
