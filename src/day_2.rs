pub fn valid_game(game: Game, rules: GameRules) -> bool {
    game.max_red <= rules.red && game.max_green <= rules.green && game.max_blue <= rules.blue
}

pub struct GameRules {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

pub struct Game {
    pub max_red: usize,
    pub max_green: usize,
    pub max_blue: usize,
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
                    v => panic!("what kind of color is this?? ({})", v),
                }
            }
        }
        Game {
            max_red,
            max_green,
            max_blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::advent;

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
