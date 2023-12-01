use regex::Regex;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
fn main() {
    let file = File::open("input.data").unwrap();
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    let total_part1 = day1(&lines, false);
    let total_part2 = day1(&lines, true);
    println!("Part 1 {}", total_part1);
    println!("Part 2 {}", total_part2);
}

fn day1(lines: &[String], part2: bool) -> i32 {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let fixed_lines:Vec<String> = lines
        .iter()
        .map(|line| {
            line.replace("oneight", "oneeight")
                .replace("twone", "twoone")
                .replace("eightwo", "eighttwo")
                .replace("sevenine", "sevennine")
                .replace("fiveight", "fiveeight")
                .replace("eighthree", "eightthree")
        })
        .collect();

    fixed_lines
        .iter()
        .map(|line| {
            let y = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)|(\d)")
                .unwrap()
                .captures_iter(line)
                .map(|capture| {
                    match capture[0].to_string().parse::<i32>() {
                        Ok(val) => val,
                        Err(_) => if part2 {(nums.iter().position(|p| p == &&capture[0])).unwrap() as i32 + 1 } else {0},
                    }
                }).filter( |x| *x != 0)
                .collect::<Vec<_>>();
            (y.first().unwrap() * 10) + y.last().unwrap()
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1(){
        let input = ["ab1bc4jas2five".to_string()];
        assert_eq!(day1(&input, false), 12);
    }

    #[test]
    fn part2(){
        let input = ["ab1bc4jas2five".to_string()];
        assert_eq!(day1(&input, true), 15);
    }
}