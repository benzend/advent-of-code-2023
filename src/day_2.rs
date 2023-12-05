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
