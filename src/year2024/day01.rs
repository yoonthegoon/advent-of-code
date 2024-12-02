use crate::result::Result;
use regex::Regex;

fn part1(lines: Vec<String>) -> Result<i32> {
    let mut diffs = Vec::new();
    let mut left = Vec::new();
    let mut right = Vec::new();

    let re = Regex::new(r"^(?<left>\d+) +(?<right>\d+)$")?;
    for line in lines {
        let caps = re.captures(&line).unwrap();
        left.push(*&caps["left"].parse::<i32>()?);
        right.push(*&caps["right"].parse::<i32>()?);
    }

    left.sort();
    right.sort();

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
        println!("{}", part1(get_lines(1).unwrap()).unwrap());
    }
}
