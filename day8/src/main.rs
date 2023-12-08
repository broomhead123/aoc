use num::Integer;
use std::collections::HashMap;
use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day8(&lines);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day8part2(&lines);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

fn day8(lines: &[String]) -> u64 {
    let path = &lines[0].trim();
    let mut map = HashMap::new();
    lines[2..].iter().for_each(|line| {
        map.insert(&line[0..=2], (&line[7..=9], &line[12..=14]));
    });
    let mut cur = "AAA";

    for (count, c) in path.chars().cycle().enumerate() {
        if cur == "ZZZ" {
            return count as u64;
        }
        if c == 'L' {
            cur = map.get(cur).unwrap().0;
        } else {
            cur = map.get(cur).unwrap().1;
        }
    }
    0
}

// fn day8part2_brute(lines: &[String]) -> u64 {
//     let path = &lines[0].trim();
//     let mut map = HashMap::new();
//     let mut starts: Vec<&str> = vec![];
//     lines[2..].iter().for_each(|line| {
//         map.insert(&line[0..=2], (&line[7..=9], &line[12..=14]));

//         if line[2..=2].eq("A") {
//             starts.push(&line[0..=2]);
//         }
//     });
//     for (count, c) in path.chars().cycle().enumerate() {
//         let res = starts
//             .iter_mut()
//             .map(|s| {
//                 // if s[2..=2].eq("Z") {
//                 //     if c == 'L' {
//                 //         *s = map.get(s).unwrap().0;
//                 //     } else {
//                 //         *s = map.get(s).unwrap().1;
//                 //     }
//                 //     true
//                 // } else {
//                 //     if c == 'L' {
//                 //         *s = map.get(s).unwrap().0;
//                 //     } else {
//                 //         *s = map.get(s).unwrap().1;
//                 //     }
//                 //     false
//                 // }
//                 if c == 'L' {
//                     *s = map.get(s).unwrap().0;
//                 } else {
//                     *s = map.get(s).unwrap().1;
//                 }
//                 s[2..=2].eq("Z")
//             })
//             .collect::<Vec<_>>();
//         let trues = res.iter().filter(|f| **f).collect::<Vec<_>>();
//         if trues.len() == starts.len() {
//             return count as u64 + 1;
//         }
//     }
//     0
// }

fn day8part2(lines: &[String]) -> u64 {
    let path = &lines[0].trim();
    let mut map = HashMap::new();
    let mut starts: Vec<&str> = vec![];
    lines[2..].iter().for_each(|line| {
        map.insert(&line[0..=2], (&line[7..=9], &line[12..=14]));

        if line[2..=2].eq("A") {
            starts.push(&line[0..=2]);
        }
    });

    let path_lens: Vec<_> = starts
        .iter_mut()
        .map(|s| {
            for (count, c) in path.chars().cycle().enumerate() {
                if s[2..=2].eq("Z") {
                    return count as u64;
                }
                if c == 'L' {
                    *s = map.get(s).unwrap().0;
                } else {
                    *s = map.get(s).unwrap().1;
                }
            }
            0
        })
        .collect::<Vec<_>>();
    path_lens.iter().fold(1, |a, &b| a.lcm(&b))
}

#[cfg(test)]
mod tests {
    use super::*;
    // Extended test.data from reddit instead of the usual small one from the puzzle
    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day8(&lines), 6);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test2.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day8part2(&lines), 6);
    }
}
