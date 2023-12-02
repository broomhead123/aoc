use regex::Regex;
use std::time::Instant;

fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1: i32 = day1(&lines, false);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }
    {
        let now = Instant::now();
        let total_part2 = day1(&lines, true);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

fn day1(lines: &[String], part2: bool) -> i32 {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let fixed_lines: Vec<String> = if part2 {
        // Fixup lines that the later regex can't match due to overlap
        // this only fixes issues found in my input data
        lines
            .iter()
            .map(|line| {
                line.replace("oneight", "oneeight")
                    .replace("twone", "twoone")
                    .replace("eightwo", "eighttwo")
                    .replace("sevenine", "sevennine")
                    .replace("fiveight", "fiveeight")
                    .replace("eighthree", "eightthree")
            })
            .collect()
    } else {
        lines.to_vec()
    };

    let regex = if part2 {
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)|(\d)")
    } else {
        Regex::new(r"(\d)")
    };

    fixed_lines
        .iter()
        .map(|line| {
            let numbers = regex
                .as_ref()
                .unwrap()
                .captures_iter(line)
                .map(|capture| {
                    // For all captured string either diretly convert to a i32
                    // or if doing part2 lookup the string in nums and return the index + 1
                    // to get the i32 equivalent
                    match capture[0].to_string().parse::<i32>() {
                        Ok(val) => val,
                        Err(_) => {
                            if part2 {
                               i32::try_from((nums.iter().position(|p| p == &&capture[0])).unwrap()).unwrap() + 1
                            } else {
                                0 // 0 for part1, these will are filtered out later
                            }
                        }
                    }
                })
                .filter(|x| *x != 0)
                .collect::<Vec<_>>();
            // Convert first and last digits into a single value
            (numbers.first().unwrap() * 10) + numbers.last().unwrap()
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = ["ab1bc4jas2five".to_string()];
        assert_eq!(day1(&input, false), 12);
    }

    #[test]
    fn part2() {
        let input = ["ab1bc4jas2five".to_string()];
        assert_eq!(day1(&input, true), 15);
    }
}
