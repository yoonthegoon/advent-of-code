use crate::result::Result;
use regex::Regex;

fn get_left_right(lines: Vec<String>) -> Result<(Vec<i32>, Vec<i32>)> {
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

fn part1(lines: Vec<String>) -> Result<i32> {
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

fn part2(lines: Vec<String>) -> Result<i32> {
    let (mut left, mut right) = get_left_right(lines.clone())?;

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
    use crate::get_lines;

    #[test]
    fn test_part1() {
        let lines = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];
        let example = part1(lines).unwrap();
        assert_eq!(example, 11);

        let lines = get_lines(1).unwrap();
        let answer = part1(lines).unwrap();
        println!("{}", answer);
    }

    #[test]
    fn test_part2() {
        let lines = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];
        let example = part2(lines).unwrap();
        assert_eq!(example, 31);

        let lines = get_lines(1).unwrap();
        let answer = part2(lines).unwrap();
        println!("{}", answer);
    }
}
