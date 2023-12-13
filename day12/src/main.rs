use itertools::Itertools;
use memoize::memoize;
use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day12(&lines, 1);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day12(&lines, 5);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

#[memoize]
fn rec(springs: Vec<char>, pattern: Vec<i64>) -> i64 {
    let mut trimmed: Vec<_> = springs.clone();
    if pattern.iter().sum::<i64>() > trimmed.iter().filter(|x| **x == '?' || **x == '#').count() as i64 {
        return 0;
    }
    while !trimmed.is_empty() && (trimmed[0] == '.') {
        trimmed.remove(0);
    }
    let mut total = 0;

    // End cases
    if trimmed.is_empty() {
        if pattern.is_empty() {
            return 1;
        }
        return 0;
    }
    if pattern.is_empty() {
        if trimmed.contains(&'#') {
            return 0;
        }
        return 1;
    }
    let mut current_pattern = pattern[0];

    if trimmed[0] == '?' {
        // but recurse as dot
        total += rec(trimmed[1..].to_vec(), pattern.clone());
        trimmed[0] = '#';

    }

    if trimmed[0] == '#' {
            //Process as much of the pattern
        while current_pattern != 0 {
            if trimmed.is_empty() {
                return total;
            }
            if trimmed[0] == '#' || trimmed[0] == '?' {
                trimmed.remove(0);
                current_pattern -= 1;
            } else {
                return total;
            }
        }
        if current_pattern == 0 && pattern.len() == 1 && trimmed.is_empty(){
            return total + 1;
        }
        if !trimmed.is_empty() && (trimmed[0] == '?' || trimmed[0] == '.') {
            return total + rec(trimmed[1..].to_vec(), pattern[1..].to_vec());
        }
        return total;
    } 
    total + rec(trimmed[1..].to_vec(), pattern.clone())
}

fn day12(lines: &[String], copies: i64) -> i64 {
    lines
        .iter()
        .map(|line| {
            let splits: Vec<_> = line.split(' ').collect();
            let springs = std::iter::repeat(splits[0].to_string())
                .take(copies as usize)
                .join("?");
            let pattern: Vec<i64> = std::iter::repeat(splits[1].to_string())
                .take(copies as usize)
                .join(",")
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            rec(springs.chars().collect(), pattern)
        })
        .collect::<Vec<i64>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day12(&lines, 1), 21);
    }
    #[test]
    fn part1a() {
        assert_eq!(day12(&["???.### 1,1,3".to_string()], 1), 1);
    }

    #[test]
    fn part2single1() {
        assert_eq!(day12(&["???.### 1,1,3".to_string()], 5), 1);
    }

    #[test]
    fn part2single2() {
        assert_eq!(day12(&[".??..??...?##. 1,1,3".to_string()], 1), 4); //16384);
    }
    
    #[test]
    fn part2single3() {
        assert_eq!(day12(&["?#?#?#?#?#?#?#? 1,3,1,6".to_string()], 5), 1);
    }
    #[test]
    fn part2single4() {
        assert_eq!(day12(&["????.#...#... 4,1,1".to_string()], 5), 16);
    }
    #[test]
    fn part2single5() {
        assert_eq!(day12(&["????.######..#####. 1,6,5".to_string()], 5), 2500);
    }
    #[test]
    fn part2single6() {
        assert_eq!(day12(&["?###???????? 3,2,1".to_string()], 5), 506_250);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day12(&lines, 5), 525_152);
    }
}
