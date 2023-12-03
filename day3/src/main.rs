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
            let mut num: i32 = 0;
            let mut start_pos: usize = 0;
            let mut nums: Vec<NumPos> = vec![];
            let _ = line
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    match c.to_string().parse::<i32>() {
                        Ok(c) => {if num == 0 {start_pos = i}; num = num*10 + c},
                        Err(_) => {
                            if num != 0 {
                                let len: usize = i - start_pos;
                                nums.push(NumPos {
                                    column: (i - len).try_into().unwrap(),
                                    length: len.try_into().unwrap(),
                                    num,
                                });
                                num = 0;
                            }
                        }
                    };
                    // Reached the end of the line, may have been parsing a number so finish that
                    if i == line.len() - 1 && num != 0 {
                        let len: usize = (num.checked_ilog10().unwrap_or(0) + 1).try_into().unwrap();
                        nums.push(NumPos {
                            column: (i - len + 1).try_into().unwrap(),
                            length: len.try_into().unwrap(),
                            num,
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

fn get_nums_for_index(index: usize, numpos: &[Vec<NumPos>], i: i32, a: &mut Vec<i32>, lines: &[String]) {
    if index == 0 {
        let mut b = find_nums(numpos, index + 1, i);
        a.append(&mut b);
    } else if index == lines.len() - 1 {
        let mut c = find_nums(numpos, index - 1, i);
        a.append(&mut c);
    } else {
        let mut b = find_nums(numpos, index - 1, i);
        let mut c = find_nums(numpos, index + 1, i);
        a.append(&mut b);
        a.append(&mut c);
    }
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
                            let mut a: Vec<i32> = find_nums(&numpos, index, i);
                            get_nums_for_index(index, &numpos, i, &mut a, lines);
                            Some(a.iter().sum::<i32>())
                        }
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
                .iter()
                .filter_map(|&x| x)
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
                            get_nums_for_index(index, &numpos, i, &mut a, lines);
                            // Any '*' with 2 nums is a gear so calculate
                            // gear ratio
                            if ch == '*' && a.len() == 2 {
                                Some(a[0] * a[1])
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
                .iter()
                .filter_map(|&x| x)
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
