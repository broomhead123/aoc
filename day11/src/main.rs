use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day11(&lines, 2);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day11(&lines, 1_000_000);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Galaxy {
    x: i64,
    y: i64,
}

fn day11(lines: &[String], expansion_size: i64) -> i64 {
    // Add extra lines
    let y_space: Vec<_> = lines
        .iter()
        .enumerate()
        .map(|(index, line): (usize, &String)| {
            if line.chars().all(|f| f == '.') {
                index as i64
            } else {
                -1
            }
        })
        .filter(|x| x >= &0)
        .collect();

    let transposed: Vec<Vec<_>> = (0..lines[0].len())
        .map(|col| {
            (0..lines.len())
                .map(|row| lines[row].chars().collect::<Vec<_>>()[col])
                .collect()
        })
        .collect();
    let x_space: Vec<_> = transposed
        .iter()
        .enumerate()
        .map(|(index, line)| {
            if line.iter().all(|f| *f == '.') {
                index as i64
            } else {
                -1
            }
        })
        .filter(|x| x >= &0)
        .collect();

    let galaxies: Vec<Galaxy> = lines
        .iter()
        .enumerate()
        .flat_map(|(index, line)| {
            line.chars()
                .enumerate()
                .filter(|x| x.1 == '#')
                .map(|(offset, _char)| Galaxy {
                    y: (index) as i64,
                    x: (offset) as i64,
                })
                .collect::<Vec<Galaxy>>()
        })
        .collect();
    println!("{x_space:?} {y_space:?}");
    let distances: Vec<i64> = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g)| {
            galaxies[i + 1..]
                .iter()
                .map(|g2| {
                    let x_count = x_space
                        .iter()
                        .filter(|x| {
                            if g.x > g2.x {
                                (g2.x..=g.x).contains(x)
                            } else {
                                (g.x..=g2.x).contains(x)
                            }
                        })
                        .count() as i64;
                    let y_count =
                        y_space.iter().filter(|y| {if g.y > g2.y {
                            (g2.y..=g.y).contains(y)
                        } else {
                            (g.y..=g2.y).contains(y)
                        }}).count() as i64;

                    (g2.x - g.x).abs() + x_count * expansion_size - x_count
                    + (g2.y - g.y).abs()
                    + y_count * expansion_size
                    - y_count
                })
                .collect::<Vec<_>>()
        })
        .collect();
    distances.iter().sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day11(&lines, 2), 374);
    }

    #[test]
    fn part2_10() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day11(&lines, 10), 1030);
    }

    #[test]
    fn part2_100() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day11(&lines, 100), 8410);
    }
}
