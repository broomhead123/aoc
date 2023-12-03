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
#[allow(clippy::too_many_lines)]
fn day3(lines: &[String]) -> i32 {
    let numpos = lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let mut num = String::new();
            let mut nums: Vec<((i32, i32, i32), i32)> = vec![];
            let _ = line
                .chars()
                .enumerate()
                .map(|(i, c)| {match c.to_string().parse::<i32>() {
                    Ok(c) => num += &c.to_string(),
                    Err(_) => {
                        if !num.is_empty() {
                            nums.push((
                                (
                                    index.try_into().unwrap(),
                                    (i - num.len()).try_into().unwrap(),
                                    num.len().try_into().unwrap(),
                                ),
                                num.parse::<i32>().unwrap(),
                            ));
                            num = String::new();
                        }
                    }};
                    
                    if i == line.len()-1 && !num.is_empty() {
                        nums.push((
                            (
                                index.try_into().unwrap(),
                                (i - num.len() + 1).try_into().unwrap(),
                                num.len().try_into().unwrap(),
                            ),
                            num.parse::<i32>().unwrap()));
                    }
                
                })
                .collect::<Vec<_>>();
            nums
        })
        .collect::<Vec<_>>();

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            line
                .chars()
                .enumerate()
                .map(|(ix, ch)| {
                    let i: i32 = ix.try_into().unwrap();
                    match ch {
                        '*' | '#' | '$' | '+' | '%' | '=' | '-' | '&' | '@' | '/' => {
                            if index == 0 {
                                let b:i32 = numpos[index]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>().iter().sum();
                                let c:i32 = numpos[index + 1]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>().iter().sum();
                               b + c
                            } else if index == lines.len() - 1 {
                                let b:i32 = numpos[index]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>().iter().sum();
                                let c:i32 = numpos[index  - 1]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>().iter().sum();
                                b + c
                            } else {
                                let a:i32 = numpos[index - 1]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>().iter().sum();
                                let b:i32 = numpos[index]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>().iter().sum();
                                let c:i32 = numpos[index + 1]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>().iter().sum();
                                a + b + c
                            }
                        }
                        _ => 0,
                    }
                })
                .collect::<Vec<_>>().iter().sum()
        })
        .collect::<Vec<_>>().iter().sum()
}


#[allow(clippy::too_many_lines)]
fn day3part2(lines: &[String]) -> i32 {
    let numpos = lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let mut num = String::new();
            let mut nums: Vec<((i32, i32, i32), i32)> = vec![];
            let _ = line
                .chars()
                .enumerate()
                .map(|(i, c)| {match c.to_string().parse::<i32>() {
                    Ok(c) => num += &c.to_string(),
                    Err(_) => {
                        if !num.is_empty() {
                            nums.push((
                                (
                                    index.try_into().unwrap(),
                                    (i - num.len()).try_into().unwrap(),
                                    num.len().try_into().unwrap(),
                                ),
                                num.parse::<i32>().unwrap(),
                            ));
                            num = String::new();
                        }
                    }};
                    
                    if i == line.len()-1 && !num.is_empty() {
                        nums.push((
                            (
                                index.try_into().unwrap(),
                                (i - num.len() + 1).try_into().unwrap(),
                                num.len().try_into().unwrap(),
                            ),
                            num.parse::<i32>().unwrap()));
                    }
                
                })
                .collect::<Vec<_>>();
            nums
        })
        .collect::<Vec<_>>();

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            line
                .chars()
                .enumerate()
                .map(|(ix, ch)| {
                    let i: i32 = ix.try_into().unwrap();
                    match ch {
                        '*'  => {
                            if index == 0 {
                                let mut a = numpos[index]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>();
                                let mut b = numpos[index + 1]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    
                                    .collect::<Vec<_>>();
                                a.append(&mut b);
                                a.retain(|x: &i32| *x != 0);

                                if ch == '*' && a.len() == 2 {
                                    a[0] * a[1]
                                } else {
                                    a.iter().sum()
                                }
                            } else if index == lines.len() - 1 {
                                let mut b = numpos[index]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    
                                    .collect::<Vec<_>>();
                                let mut c = numpos[index  - 1]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>();
                                b.append(&mut c);
                                b.retain(|x: &i32| *x != 0);

                                if ch == '*' && b.len() == 2 {
                                    b[0] * b[1]
                                } else {
                                    b.iter().sum()
                                }
                            } else {
                                let mut a = numpos[index - 1]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    .collect::<Vec<_>>();
                                let mut b = numpos[index]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    
                                    .collect::<Vec<_>>();
                                let mut c = numpos[index + 1]
                                    .iter()
                                    .filter(|c| {
                                        (i - c.0 .1).abs() <= 1 || i >= c.0 .1 && i <= c.0 .1 + c.0 .2
                                    }).map(| x | x.1)
                                    
                                    .collect::<Vec<_>>();
                                a.append(&mut b);
                                a.append(&mut c);
                                a.retain(|x: &i32| *x != 0);
                                if ch == '*' && a.len() == 2 {
                                    a[0] * a[1]
                                } else {
                                    0
                                }
                            }
                        }
                        _ => 0,
                    }
                })
                .collect::<Vec<_>>().iter().sum()
        })
        .collect::<Vec<_>>().iter().sum()
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
        let i = "467.114+22\n\
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
        let i = "467.114.22\n\
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
        let i = "......+..\n\
                       ...12.345.\n\
                       ...+......\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 357);
    }

    #[test]
    fn mid_line2() {
        let i = ".....12...\n\
                       ...+..+...\n\
                       ..123.....\n";
        let lines: Vec<String> = i.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day3(&lines), 135);
    }

}
