use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day14(&lines);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day14part2(&lines);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

fn day14(lines: &[String]) -> i64 {
    lines[0]
        .split(',')
        .map(|s| {
            let mut hash: i64 = 0;
            for c in s.chars() {
                hash += c as i64;
                hash *= 17;
                hash %= 256;
            }
            hash
        })
        .sum()
}

fn day14part2(lines: &[String]) -> i64 {
    let mut boxes: [Vec<_>; 256] = [(); 256].map(|()| Vec::<(&str, i64)>::new());
    let _ = lines[0]
        .split(',')
        .map(|s| {
            let mut hash: i64 = 0;
            let label = s.split(|c| c == '-' || c == '=').collect::<Vec<_>>()[0];
            let focal_length = if s.contains("=") {
                s.split(|c| c == '-' || c == '=').collect::<Vec<_>>()[1]
                    .parse::<i64>()
                    .unwrap()
            } else {
                -1
            };
            for c in label.chars() {
                hash += c as i64;
                hash *= 17;
                hash %= 256;
            }
            if focal_length == -1 {
                if boxes[hash as usize].iter().any(|x| x.0 == label) {
                    boxes[hash as usize].remove(
                        boxes[hash as usize]
                            .iter()
                            .position(|x| x.0 == label)
                            .unwrap(),
                    );
                    boxes[hash as usize].retain(|x| x.0 != "");
                }
            } else if boxes[hash as usize].iter().any(|x| x.0 == label) {
                let pos = boxes[hash as usize]
                    .iter()
                    .position(|x| x.0 == label)
                    .unwrap();
                //boxes[hash as usize].remove(pos);
                boxes[hash as usize][pos] = (label, focal_length);
            } else {
                boxes[hash as usize].push((label, focal_length));
            }
        })
        .collect::<Vec<()>>();
    i64::try_from(
        boxes
            .iter()
            .enumerate()
            .map(|b| {
                b.1.iter()
                    .enumerate()
                    .map(|l| (b.0 + 1) * (l.0 + 1) * (l.1 .1 as usize))
                    .sum::<usize>()
            })
            .sum::<usize>(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1hash() {
        let file = include_str!("../test.data");
        assert_eq!(day14(&["HASH".to_string()]), 52);
    }
    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day14(&lines), 1320);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day14part2(&lines), 145);
    }
}
