use std::{time::Instant, usize};
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day13(&lines, false);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day13(&lines, true);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

fn row_diff_count(vec_a: Vec<char>, vec_b: Vec<char>) -> i64 {
    vec_a
        .into_iter()
        .zip(vec_b)
        .map(|(a, b)| if a == b { ' ' } else { '#' })
        .filter(|c| *c == '#')
        .count() as i64
}

fn day13(lines: &[String], part2: bool) -> i64 {
    let mut patterns: Vec<Vec<Vec<char>>> = vec![];
    let mut pattern: Vec<Vec<char>> = vec![];
    for line in lines {
        if line.is_empty() && !pattern.is_empty() {
            patterns.push(pattern.clone());
            pattern.clear();
        } else if !line.is_empty() {
            pattern.push(line.chars().collect());
        }
    }
    if !pattern.is_empty() {
        patterns.push(pattern.clone());
    }

    let row = patterns
        .iter()
        .map(|p| {
            let mut prev_rows = 0;
            for i in 1..p.len() {
                let mut start: usize = 0;
                if i < (p.len()) / 2 + 1 {
                    start = 0;
                } else {
                    start = i - (p.len() - i);
                }
                let mut end: usize = 0;
                if i < (p.len() / 2) + 1 {
                    end = i + (i - 1);
                } else {
                    end = p.len() - 1;
                }
                let mut lhs = p[start..i].to_vec();
                lhs.reverse();
                assert!(lhs.len() == p[i..=end].to_vec().len());
                if part2 {
                    let mut diff = 0;
                    for r in 0..lhs.len() {
                        diff += row_diff_count(lhs[r].clone(), p[i..=end].to_vec()[r].clone());
                    }
                    if diff == 1 {
                        prev_rows = i;
                        break;
                    }
                } else {
                    if lhs == p[i..=end].to_vec() {
                        prev_rows = i;
                        break;
                    }
                }
            }
            100 * prev_rows
        })
        .collect::<Vec<usize>>();

    let col = patterns
        .iter()
        .map(|ip| {
            let mut prev_cols = 0;
            let p: Vec<_> = (0..ip[0].len())
                .map(|i| ip.iter().map(|inner| inner[i]).collect::<Vec<char>>())
                .collect();
            for i in 1..p.len() {
                let mut start: usize = 0;
                if i < (p.len()) / 2 + 1 {
                    start = 0;
                } else {
                    start = i - (p.len() - i);
                }
                let mut end: usize = 0;
                if i < (p.len() / 2) + 1 {
                    end = i + (i - 1);
                } else {
                    end = p.len() - 1;
                }
                let mut lhs = p[start..i].to_vec();
                lhs.reverse();
                assert!(lhs.len() == p[i..=end].to_vec().len());
                if part2 {
                    let mut diff = 0;
                    for r in 0..lhs.len() {
                        diff += row_diff_count(lhs[r].clone(), p[i..=end].to_vec()[r].clone());
                    }
                    if diff == 1 {
                        prev_cols = i;
                        break;
                    }
                } else {
                    if lhs == p[i..=end].to_vec() {
                        prev_cols = i;
                        break;
                    }
                }
            }
            prev_cols
        })
        .collect::<Vec<usize>>();
    i64::try_from(row.iter().sum::<usize>()).unwrap()
        + i64::try_from(col.iter().sum::<usize>()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day13(&lines, false), 405);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day13(&lines, true), 400);
    }
}
