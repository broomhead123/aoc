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
    lines[0].split(',').map(|s| hash_str(s.as_bytes())).sum()
}

fn day14part2(lines: &[String]) -> i64 {
    let mut boxes: [Vec<_>; 256] = [(); 256].map(|()| Vec::<(&str, i64)>::new());
    let _ = lines[0]
        .split(',')
        .map(|s| {
            let mut focal_length:Option<i64> = None;

            let label = if &s[s.len()-1..] == "-" {
                &s[0..s.len()-1]
            } else {
                focal_length = Some(s[s.len() - 1..].parse::<i64>().unwrap());
                &s[0..s.len() - 2]
            };

            let hash = usize::try_from(hash_str(label.as_bytes())).unwrap();
            let pos = boxes[hash].iter().position(|x| x.0 == label);
            if focal_length.is_none() {
                if let Some(pos) = pos {
                    boxes[hash].remove(pos);
                    boxes[hash as usize].retain(|x| !x.0.is_empty());
                }
            } else if pos.is_some() {
                let pos = pos.unwrap();
                boxes[hash][pos] = (label, focal_length.unwrap());
            } else {
                boxes[hash].push((label, focal_length.unwrap()));
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
                    .map(|l| (b.0 + 1) * (l.0 + 1) * usize::try_from(l.1 .1).unwrap())
                    .sum::<usize>()
            })
            .sum::<usize>(),
    )
    .unwrap()
}

fn hash_str(label: &[u8]) -> i64 {
    let mut hash: i64 = 0;
    let len = label.len();
    let mut i = 0;
    while i < len {
        hash += i64::from(label[i]);
        hash *= 17;
        hash %= 256;
        i += 1;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1hash() {
        assert_eq!(hash_str("HASH".as_bytes()), 52);
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
