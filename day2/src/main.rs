use regex::Regex;

fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    use std::time::Instant;
    {
        let now = Instant::now();
        let total_part1: i32 = day2(&lines);
        println!("Part 1 {}", total_part1);
        println!("Done in: {:.2?}", now.elapsed());
    }
    {
        let now = Instant::now();
        let total_part2 = day2part2(&lines);
        println!("Part 2 {}", total_part2);
        println!("Done in: {:.2?}", now.elapsed());
    }
}

fn day2(lines: &[String]) -> i32 {
    let r = Regex::new(r"((\d+) red)|((\d+) green)|((\d+) blue)");
    lines
        .iter()
        .map(|line| {
            let split = line.split(':').collect::<Vec<_>>();
            let game = split[0].split(' ').collect::<Vec<_>>()[1].parse::<i32>().unwrap();
            let grabs = split[1].split(';').collect::<Vec<_>>();
            let valid = grabs
                .iter()
                .map(|grab| {
                    let mut red_count = 0;
                    let mut green_count = 0;
                    let mut blue_count = 0;
                    let _ = r
                        .as_ref()
                        .unwrap()
                        .captures_iter(grab)
                        .map(|c| {
                            if c.get(1).is_some() {
                                red_count += c[2].to_string().parse::<i32>().unwrap();
                            } else if c.get(3).is_some() {
                                green_count += c[4].to_string().parse::<i32>().unwrap();
                            } else {
                                blue_count += c[6].to_string().parse::<i32>().unwrap();
                            }
                        })
                        .collect::<Vec<_>>();
                    red_count <= 12 && green_count <= 13 && blue_count <= 14 
                })
                .collect::<Vec<_>>().iter().all(|x| *x);
            if valid {
                game
            } else {
                0
            }
        })
        .collect::<Vec<_>>().iter().sum()
}

fn day2part2(lines: &[String]) -> i32 {
    let r = Regex::new(r"((\d+) red)|((\d+) green)|((\d+) blue)");
    lines
        .iter()
        .map(|line| {
            let split = line.split(':').collect::<Vec<_>>();
            let grabs = split[1].split(';').collect::<Vec<_>>();
            let mut red_highest = 0;
            let mut green_highest = 0;
            let mut blue_highest = 0;
            let _ = grabs
                .iter()
                .map(|grab| {
                    let _ = r
                        .as_ref()
                        .unwrap()
                        .captures_iter(grab)
                        .map(|c: regex::Captures<'_>| {
                            if c.get(1).is_some() {
                                let red = c[2].to_string().parse::<i32>().unwrap();
                                if red_highest < red {red_highest = red};
                            } else if c.get(3).is_some() {
                                let green = c[4].to_string().parse::<i32>().unwrap();
                                if green_highest < green {green_highest = green};
                            } else {
                                let blue = c[6].to_string().parse::<i32>().unwrap();
                                if blue_highest < blue {blue_highest = blue};
                            }
                        })
                        .collect::<Vec<_>>();
                })
                .collect::<Vec<_>>();
                red_highest * blue_highest * green_highest

        })
        .collect::<Vec<_>>().iter().sum()
}
