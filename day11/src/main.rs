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
    let better_lines: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    // Add extra lines
    let y_space: Vec<_> = better_lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            if line.iter().all(|f| *f == '.') {
                i64::try_from(index).unwrap()
            } else {
                -1
            }
        })
        .filter(|x| x >= &0)
        .collect();
    let mut x_space = vec![];
    (0..better_lines[0].len()).for_each(|i| {
        if better_lines.iter().map(|l| l[i]).all(|c| c == '.') {
            x_space.push(i64::try_from(i).unwrap());
        }
    });

    let galaxies: Vec<Galaxy> = better_lines
        .iter()
        .enumerate()
        .flat_map(|(index, line)| {
            line.iter()
                .enumerate()
                .filter(|x| *x.1 == '#')
                .map(|(offset, _char)| Galaxy {
                    y: i64::try_from(index).unwrap(),
                    x: i64::try_from(offset).unwrap(),
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
                    let x_count = i64::try_from(x_space
                        .iter()
                        .filter(|x| {
                            if g.x > g2.x {
                                **x >= g2.x && **x <= g.x
                            } else {
                                **x >= g.x && **x <= g2.x
                            }
                        })
                        .count()).unwrap();
                    let y_count = i64::try_from(y_space
                        .iter()
                        .filter(|y| {
                            if g.y > g2.y {
                                **y >= g2.y && **y <= g.y
                            } else {
                                **y >= g.y && **y <= g2.y
                            }
                        })
                        .count()).unwrap();

                    (g2.x - g.x).abs() + x_count * expansion_size
                        - x_count
                        + (g2.y - g.y).abs()
                        + y_count * expansion_size
                        - y_count
                })
                .collect::<Vec<i64>>()
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
