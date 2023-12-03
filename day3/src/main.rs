use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1: i32 = day3(&lines);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }
    {
        let now = Instant::now();
        let total_part2 = day3part2(&lines);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

struct NumPos {
    column: i32,
    length: i32,
    num: i32,
}

fn get_num_positions(lines: &[String]) -> Vec<Vec<NumPos>> {
    let num_pos = lines
        .iter()
        .map(|line| {
            let mut num = String::new();
            let mut nums: Vec<NumPos> = vec![];
            let _ = line
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    match c.to_string().parse::<i32>() {
                        Ok(c) => num += &c.to_string(),
                        Err(_) => {
                            if !num.is_empty() {
                                nums.push(NumPos {
                                    column: (i - num.len()).try_into().unwrap(),
                                    length: num.len().try_into().unwrap(),
                                    num: num.parse::<i32>().unwrap(),
                                });
                                num = String::new();
                            }
                        }
                    };

                    if i == line.len() - 1 && !num.is_empty() {
                        nums.push(NumPos {
                            column: (i - num.len() + 1).try_into().unwrap(),
                            length: num.len().try_into().unwrap(),
                            num: num.parse::<i32>().unwrap(),
                        });
                    }
                })
                .collect::<Vec<_>>();
            nums
        })
        .collect::<Vec<_>>();
    num_pos
}

fn find_nums(numpos: &[Vec<NumPos>], index: usize, i: i32) -> Vec<i32> {
    numpos[index]
        .iter()
        .filter(|c| (i - c.column).abs() <= 1 || i >= c.column && i <= c.column + c.length)
        .map(|x| x.num)
        .collect::<Vec<_>>()
}

fn day3(lines: &[String]) -> i32 {
    let numpos = get_num_positions(lines);

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            line.chars()
                .enumerate()
                .map(|(ix, ch)| {
                    let i: i32 = ix.try_into().unwrap();
                    match ch {
                        '*' | '#' | '$' | '+' | '%' | '=' | '-' | '&' | '@' | '/' => {
                            let a:i32 = find_nums(&numpos, index, i).iter().sum();
                            if index == 0 {
                                let c: i32 = find_nums(&numpos, index + 1, i).iter().sum();
                                a + c
                            } else if index == lines.len() - 1 {
                                let c: i32 = find_nums(&numpos, index - 1, i).iter().sum();
                                a + c
                            } else {
                                let b: i32 = find_nums(&numpos, index - 1, i).iter().sum();
                                let c: i32 = find_nums(&numpos, index + 1, i).iter().sum();
                                a + b + c
                            }
                        }
                        _ => 0,
                    }
                })
                .collect::<Vec<_>>()
                .iter()
                .sum()
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn day3part2(lines: &[String]) -> i32 {
    let numpos = get_num_positions(lines);

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            line.chars()
                .enumerate()
                .map(|(ix, ch)| {
                    let i: i32 = ix.try_into().unwrap();
                    match ch {
                        '*' => {
                            let mut a: Vec<i32> = find_nums(&numpos, index, i);
                            if index == 0 {
                                let mut b = find_nums(&numpos, index + 1, i);
                                a.append(&mut b);

                            } else if index == lines.len() - 1 {
                                let mut c = find_nums(&numpos, index - 1, i);
                                a.append(&mut c);
                            } else {
                                let mut b = find_nums(&numpos, index - 1, i);
                                let mut c = find_nums(&numpos, index + 1, i);
                                a.append(&mut b);
                                a.append(&mut c);

                            }
                            a.retain(|x: &i32| *x != 0);
                            if ch == '*' && a.len() == 2 {
                                a[0] * a[1]
                            } else {
                                0
                            }
                        }
                        _ => 0,
                    }
                })
                .collect::<Vec<_>>()
                .iter()
                .sum()
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 4361);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3part2(&lines), 467_835);
    }
    #[test]
    fn same_line() {
        let i = "467.+114..\n
                       ..........";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 114);
    }
    #[test]
    fn same_line2() {
        let i = "467..114+22\n\
                       ..........";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 136);
    }
    #[test]
    fn same_line3() {
        let i = "+22.......\n\
                       ..........";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 22);
    }

    #[test]
    fn same_line4() {
        let i = ".......22+\n\
                       ..........";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 22);
    }
    #[test]
    fn prev_line() {
        let i = "467..114.22\n\
                       .....+....\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 114);
    }

    #[test]
    fn prev_line2() {
        let i = "1234...222\n\
                       .....+....\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 0);
    }

    #[test]
    fn prev_line3() {
        let i = "12345.2222\n\
                       .....+....\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 14567);
    }

    #[test]
    fn next_line() {
        let i = ".....+....\n\
                       1234...222\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 0);
    }

    #[test]
    fn next_line2() {
        let i = ".....+....\n\
                       ...12345..\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 12345);
    }
    #[test]
    fn next_line3() {
        let i = ".....+....\n\
                       ...12.345.\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 357);
    }

    #[test]
    fn mid_line() {
        let i = ".......+..\n\
                       ...12.345.\n\
                       ...+......\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 357);
    }

    #[test]
    fn mid_line2() {
        let i = "......12...\n\
                       ...+...+...\n\
                       ...123.....\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 135);
    }
}
