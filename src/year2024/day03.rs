use crate::result::Result;
use regex::Regex;

fn get_product(factor_re: &Regex, instruction: &str) -> i32 {
    factor_re
        .captures_iter(instruction)
        .fold(1, |acc, factor| {
            acc * factor.get(0).unwrap().as_str().parse::<i32>().unwrap()
        })
}

fn part1(input: String) -> Result<i32> {
    let mut sum = 0;

    let re = Regex::new(r"(?<instruction>mul\(\d+,\d+\))")?;
    let factor_re = Regex::new(r"\d+")?;

    for cap in re.captures_iter(input.as_str()) {
        let instruction = cap.name("instruction").unwrap().as_str();
        let product = get_product(&factor_re, instruction);
        sum += product;
    }

    Ok(sum)
}

fn part2(input: String) -> Result<i32> {
    let mut sum = 0;

    let re = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|(?<instruction>mul\(\d+,\d+\))")?;
    let factor_re = Regex::new(r"\d+")?;

    let mut enabled = true;
    for cap in re.captures_iter(input.as_str()) {
        if cap.name("do").is_some() {
            enabled = true;
        } else if cap.name("dont").is_some() {
            enabled = false;
        } else if enabled {
            let instruction = cap.name("instruction").unwrap().as_str();
            let product = get_product(&factor_re, instruction);
            sum += product;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;

    #[test]
    fn test_part1() {
        let example_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let example = part1(example_input).unwrap();
        assert_eq!(example, 161);

        let input = get_input(2024, 3).unwrap();
        let answer = part1(input).unwrap();
        println!("{}", answer);
    }

    #[test]
    fn test_part2() {
        let example_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        let example = part2(example_input).unwrap();
        assert_eq!(example, 48);

        let input = get_input(2024, 3).unwrap();
        let answer = part2(input).unwrap();
        println!("{}", answer);
    }
}
