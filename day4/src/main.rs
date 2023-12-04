use std::cmp::min;
use std::collections::HashSet;
use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1: i64 = day4_hash(&lines);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }
    {
        let now = Instant::now();
        let total_part1: i64 = day4(&lines, 10);
        println!("Part 1b {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }
    {
        let now = Instant::now();
        let total_part2 = day4part2(&lines, 10);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

fn day4(lines: &[String], win_nums: usize) -> i64 {
    lines
        .iter()
        .map(|line| {
            let num_wins = calculate_line_win(line, win_nums);
            if num_wins > 0 {
                2_i64.pow(u32::try_from(num_wins).unwrap() - 1)
            } else {
                0
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn day4_hash(lines: &[String]) -> i64 {
    lines
        .iter()
        .map(|line| {
            let binding = line.split(':').collect::<Vec<_>>();
            let l = binding.last().unwrap();

            let winning_numbers: HashSet<i32> = l
                .get(1..30)
                .unwrap()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<HashSet<_>>();

            let num_to_check: HashSet<i32> = l
                .get(33..)
                .unwrap()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<HashSet<_>>();
            let num_wins: i64 =
                i64::try_from(winning_numbers.intersection(&num_to_check).count()).unwrap();
            if num_wins > 0 {
                2_i64.pow(u32::try_from(num_wins).unwrap() - 1)
            } else {
                0
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

#[derive(Debug, Clone)]
struct ScratchCard {
    matches: i64,
    copies: i64,
}

fn day4part2(lines: &[String], win_nums: usize) -> i64 {
    let mut scratch_cards: Vec<ScratchCard> = lines
        .iter()
        .map(|line| {
            let num_wins = calculate_line_win(line, win_nums);

            ScratchCard {
                matches: num_wins,
                copies: 1,
            }
        })
        .collect::<Vec<_>>();
    let len = scratch_cards.len();
    for i in 0..scratch_cards.len() {
        let card = scratch_cards[i].clone();
        if card.matches != 0 {
            for s in &mut scratch_cards[i + 1..=min(i + usize::try_from(card.matches).unwrap(), len)] {
                s.copies += card.copies;
            }
        }
    }
    scratch_cards
        .iter()
        .map(|s| s.copies)
        .collect::<Vec<_>>()
        .iter()
        .sum::<i64>()
}

fn calculate_line_win(line: &str, win_nums: usize) -> i64 {
    let binding = line.split(':').collect::<Vec<_>>();
    let l = binding.last().unwrap();
    let winning_numbers = l
        .get(1..win_nums * 3)
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let num_to_check = l
        .get(((win_nums * 3) + 3)..)
        .unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    num_to_check
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day4(&lines, 5), 13);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day4part2(&lines, 5), 30);
    }
}
