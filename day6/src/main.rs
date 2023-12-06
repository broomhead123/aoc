use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let mut lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day6(&mut lines, false);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day6(&mut lines, true);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

fn day6(lines: &mut [String], part2: bool) -> u64 {
    if part2 {
        lines[0] = lines[0].replace(' ', "");
        lines[1] = lines[1].replace(' ', "");
    }

    let times: Vec<u64> = lines[0]["Time:".len()..]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let distances: Vec<u64> = lines[1]["Distance:".len()..]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let races = times.iter().zip(distances.iter());

    let res = races
        .map(|(t, d)| {
            (1..*t)
                .filter(|i| i * (t - i) > *d)
                .collect::<Vec<_>>()
                .len()
        })
        .product::<usize>();

    res.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let mut lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day6(&mut lines, false), 288);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let mut lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day6(&mut lines, true), 71503);
    }
}
