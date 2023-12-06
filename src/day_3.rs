#[cfg(test)]
mod tests {
    #[test]
    fn test_advent_day_3_part_1_example() {
        let data = vec![
            "467..114..", // 114 isn't adjacent
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        // (line, last_character_position, value)
        let mut num_groups: Vec<(usize, usize, String)> = Vec::new();
        let mut specials: Vec<(usize, usize, String)> = Vec::new();
        for (line_pos, line_val) in data.iter().enumerate() {
            let mut num_group = "".to_string();
            for (ch_pos, ch) in line_val.char_indices() {
                let is_end_of_line = line_val.chars().count() - 1 == ch_pos;

                if ch.is_digit(10) {
                    num_group.push(ch);
                    if is_end_of_line {
                        num_groups.push((line_pos, ch_pos, num_group.clone()));
                    }
                } else if ch.to_string() == "." {
                    if num_group.chars().count() > 0 {
                        num_groups.push((line_pos, ch_pos - 1, num_group));
                        num_group = "".to_string();
                    }
                } else {
                    // is a special symbol
                    specials.push((line_pos, ch_pos, ch.to_string()));
                }
            }
        }

        assert_eq!(num_groups.iter().count(), 10);
        assert_eq!(specials.iter().count(), 6);

        assert_eq!(num_groups[0].0, 0);
        assert_eq!(num_groups[0].1, 2);

        let mut adjacents = Vec::new();
        for group in num_groups.iter() {
            for special in specials.iter() {
                if crate::day_3::is_adjacent_to_special(special, group) {
                    adjacents.push(group);
                    break;
                }
            }
        }

        assert_eq!(adjacents.len(), 8);
    }
}

pub fn is_adjacent_to_special(special: &(usize, usize, String), num_group: &(usize, usize, String)) -> bool {
    let y_distance = if special.0 > num_group.0 {
        special.0 - num_group.0
    } else {
        num_group.0 - special.0
    };

    // items on the same line will never be adjacent
    if y_distance == 0 {
        return false;
    }

    let num_group_x_range = (num_group.1, num_group.1 - (num_group.2.chars().count() - 1));

    special.1 + y_distance > num_group_x_range.0
    && num_group_x_range.1 > special.1 + y_distance
}
