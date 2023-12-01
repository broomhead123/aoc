use regex::Regex;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
fn main() {
    let file = File::open("input.data").unwrap();
    let buf = BufReader::new(file);
    let mut lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    lines = lines
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

    let split: i32 = lines
        .iter()
        .map(|line| {
            let y = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)|(\d)")
                .unwrap()
                .captures_iter(line)
                .map(|capture| {
                    match capture[0].to_string().parse::<i32>() {
                        Ok(val) => val,
                        //Err(_) => *nums2.get(&c[0]).unwrap() + 1,
                        Err(_) => (nums.iter().position(|p| p == &&capture[0])).unwrap() as i32 + 1,
                    }
                })
                .collect::<Vec<_>>();
            (y.first().unwrap() * 10) + y.last().unwrap()
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    print!("{}", split);
}
