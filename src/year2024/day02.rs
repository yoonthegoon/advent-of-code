use crate::result::Result;

fn part1(input: String) -> Result<i32> {
    let mut safe_count = 0;

    for line in input.lines() {
        let levels: Vec<_> = line.split(" ").collect();
        let mut prev_level = levels[0].parse::<i32>()?;
        let mut increasing = None;
        let mut level_count = 1;

        for &level_str in levels[1..].iter() {
            let level = level_str.parse::<i32>()?;
            let diff = level - prev_level;
            prev_level = level;
            if diff < -3 || diff == 0 || diff > 3 {
                break;
            }

            match increasing {
                None => {
                    if diff > 0 {
                        increasing = Some(true);
                    } else {
                        increasing = Some(false);
                    }
                }
                Some(true) => {
                    if diff < 0 {
                        break;
                    }
                }
                Some(false) => {
                    if diff > 0 {
                        break;
                    }
                }
            }
            level_count += 1;
        }
        if level_count == levels.len() {
            safe_count += 1;
        }
    }

    Ok(safe_count)
}

fn part2(input: String) -> Result<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        let example = part1(EXAMPLE.to_string()).unwrap();
        assert_eq!(example, 2);

        let input = get_input(2024, 2).unwrap();
        let answer = part1(input).unwrap();
        println!("{}", answer);
    }

    #[test]
    fn test_part2() {
        let example = part2(EXAMPLE.to_string()).unwrap();
        assert_eq!(example, 4);

        let input = get_input(2024, 2).unwrap();
        let answer = part2(input).unwrap();
        println!("{}", answer);
    }
}
