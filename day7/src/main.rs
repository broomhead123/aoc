use itertools::Itertools;
use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day7(&lines);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day7part2(&lines);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

fn day7(lines: &[String]) -> u64 {
    let order = "23456789TJQKA";
    let mut hands = lines
        .iter()
        .map(|l| {
            let g: Vec<_> = l.split(' ').collect();
            let mut h: Vec<char> = g[0].chars().collect();
            h.sort_by_key(|a| order.find(*a).unwrap());

            (String::from_iter(h), g[1].parse::<u64>().unwrap(), g[0])
        })
        .collect::<Vec<_>>();

    sort_hands(&mut hands, order);
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.1 * (i as u64 + 1))
        .sum()
}

fn day7part2(lines: &[String]) -> u64 {
    let order = "J23456789TQKA";
    let mut hands = lines
        .iter()
        .map(|l| {
            let g: Vec<_> = l.split(' ').collect();
            let mut h: Vec<char> = g[0].chars().collect();
            h.sort_by_key(|a| order.find(*a).unwrap());
            //Make J copy the highest most common other card
            if h[0] == 'J' {
                let repl = {
                    let mut jless: Vec<_> = h.iter_mut().filter(|c| **c != 'J').collect();
                    let cnts: Vec<_> = jless
                        .iter_mut()
                        .rev()
                        .dedup_with_count()
                        .sorted_by_key(|s| s.0)
                        .rev()
                        .collect();
                    if cnts.is_empty() {
                        ('J', 0)
                    } else {
                        (cnts[0].1.to_owned(), jless.len())
                    }
                };
                for i in 0..h.len() - repl.1 {
                    h[i] = repl.0;
                }
                h.sort_by_key(|a| order.find(*a).unwrap());
            }
            (String::from_iter(h), g[1].parse::<u64>().unwrap(), g[0])
        })
        .collect::<Vec<_>>();

    sort_hands(&mut hands, order);
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.1 * (i as u64 + 1))
        .sum()
}

fn sort_hands(hands: &mut [(String, u64, &str)], order: &str) {
    hands.sort_by_key(|h| {
        //Five of a kind
        let c: Vec<char> = h.0.chars().collect();
        let hs: Vec<usize> = h.2.chars().map(|c| order.find(c).unwrap()).collect();
        if c[0] == c[4] {
            return (9, hs, 0);
        }
        //Four of a kind
        if c[0] == c[3] || c[1] == c[4] {
            return (8, hs, 0);
        }
        //Full house
        if (c[0] == c[2] && c[3] == c[4]) || (c[2] == c[4] && c[0] == c[1]) {
            return (7, hs, 0);
        }
        //Three of a kind
        if c[0] == c[2] || c[1] == c[3] || c[2] == c[4] {
            return (6, hs, 0);
        }
        // Two pairs
        if c[0] == c[1] && (c[2] == c[3] || c[3] == c[4])
            || (c[1] == c[2] || c[0] == c[1]) && c[3] == c[4]
        {
            return (5, hs, 0);
        }
        // One pair
        if c[0] == c[1] || c[1] == c[2] || c[2] == c[3] || c[3] == c[4] {
            return (4, hs, 0);
        }
        // High card
        (0, hs, order.find(c[4]).unwrap())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day7(&lines), 6440);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day7part2(&lines), 5905);
    }
}
