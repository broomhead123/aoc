use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day9(&lines);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day9part2(&lines);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

fn day9(lines: &[String]) -> i64 {
    lines
        .iter()
        .map(|line| {
            let mut nums: Vec<i64> = line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect();
            let mut lasts: Vec<i64> = vec![];

            lasts.push(*nums.last().unwrap());
            while !nums.iter().all(|x| *x == 0) {
                nums = nums.windows(2).map(|a: &[i64]| a[1] - a[0]).collect();
                lasts.push(*nums.last().unwrap());
            }
            lasts.iter().sum::<i64>()
        })
        .sum::<i64>()
}

fn day9part2(lines: &[String]) -> i64 {
    lines
        .iter()
        .map(|line| {
            let mut nums: Vec<i64> = line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect();
            let mut firsts: Vec<i64> = vec![];

            firsts.push(*nums.first().unwrap());
            while !nums.iter().all(|x| *x == 0) {
                nums = nums.windows(2).map(|a: &[i64]| a[1] - a[0]).collect();
                firsts.push(*nums.first().unwrap());
            }

            firsts.reverse();
            firsts.into_iter().reduce(|a, b| b - a).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day9(&lines), 114);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day9part2(&lines), 2);
    }
}
