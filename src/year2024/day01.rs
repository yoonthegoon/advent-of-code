use crate::result::Result;
use regex::Regex;

fn get_left_right(lines: Vec<&str>) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left = vec![];
    let mut right = vec![];

    let re = Regex::new(r"^(?<left>\d+) +(?<right>\d+)$")?;
    for line in lines {
        let caps = re.captures(&line).unwrap();
        left.push(*&caps["left"].parse::<i32>()?);
        right.push(*&caps["right"].parse::<i32>()?);
    }

    Ok((left, right))
}

fn part1(input: String) -> Result<i32> {
    let lines: Vec<_> = input.lines().collect();
    let (mut left, mut right) = get_left_right(lines.clone())?;

    left.sort();
    right.sort();

    let mut diffs = vec![];

    for (l, r) in left.iter().zip(right.iter()) {
        let diff = l - r;
        if diff > 0 {
            diffs.push(diff);
        } else {
            diffs.push(-1 * diff);
        }
    }

    Ok(diffs.iter().sum())
}

fn part2(input: String) -> Result<i32> {
    let lines: Vec<_> = input.lines().collect();
    let (left, right) = get_left_right(lines.clone())?;

    let mut prods = vec![];

    for l in left.iter() {
        let count = right.iter().filter(|&r| l == r).count();
        let prod = l * count as i32;
        prods.push(prod);
    }

    Ok(prods.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        let example = part1(EXAMPLE.to_string()).unwrap();
        assert_eq!(example, 11);

        let input = get_input(2024, 1).unwrap();
        let answer = part1(input).unwrap();
        println!("{}", answer);
    }

    #[test]
    fn test_part2() {
        let example = part2(EXAMPLE.to_string()).unwrap();
        assert_eq!(example, 31);

        let input = get_input(2024, 1).unwrap();
        let answer = part2(input).unwrap();
        println!("{}", answer);
    }
}
