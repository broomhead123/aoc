use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day10(&lines);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day10part2(&lines);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}
#[derive(Debug, Clone, PartialEq)]

enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
    Ground,
}

#[derive(Debug, Clone, PartialEq)]
struct Cell {
    x: i64,
    y: i64,
    pipe: Pipe,
}

fn move_dir(cell: &Cell, dir: i64) -> Option<(i64, i64, i64)> {
    match dir {
        // Moving north
        0 => match cell.pipe {
            Pipe::NS | Pipe::Start => Some((-1, 0, 0)),
            Pipe::SW => Some((0, -1, 3)),
            Pipe::SE => Some((0, 1, 1)),
            _ => None,
        },
        // Moving east
        1 => match cell.pipe {
            Pipe::NW => Some((-1, 0, 0)),
            Pipe::SW => Some((1, 0, 2)),
            Pipe::EW | Pipe::Start => Some((0, 1, 1)),
            _ => None,
        },
        // Moving south
        2 => match cell.pipe {
            Pipe::NS | Pipe::Start => Some((1, 0, 2)),
            Pipe::NE => Some((0, 1, 1)),
            Pipe::NW => Some((0, -1, 3)),
            _ => None,
        },
        // Moving west
        3 => match cell.pipe {
            Pipe::EW | Pipe::Start => Some((0, -1, 3)),
            Pipe::NE => Some((-1, 0, 0)),
            Pipe::SE => Some((1, 0, 2)),
            _ => None,
        },
        _ => None,
    }
}

fn day10(lines: &[String]) -> i64 {
    let mut start: Option<Cell> = None;
    let width = lines[0].len() as i64;
    // Somehow build the puzzle
    let pipes = lines
        .iter()
        .enumerate()
        .map(|(pos, line): (usize, &String)| {
            line.chars()
                .enumerate()
                .map(|(offset, c)| match c {
                    '|' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::NS,
                    },
                    '-' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::EW,
                    },
                    'L' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::NE,
                    },
                    'J' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::NW,
                    },
                    '7' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::SW,
                    },
                    'F' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::SE,
                    },
                    'S' => {
                        start = Some(Cell {
                            x: pos as i64,
                            y: offset as i64,
                            pipe: Pipe::Start,
                        });
                        Cell {
                            x: pos as i64,
                            y: offset as i64,
                            pipe: Pipe::Start,
                        }
                    }
                    '.' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::Ground,
                    },
                    _ => todo!("later"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let ss = start.unwrap();
    //walk pipes
    let paths: Vec<_> = (0..=3)
        .map(|i| {
            let mut pstart = &pipes[(ss.x) as usize][(ss.y) as usize];
            let mut dir = i;
            let mut path_len = 0;
            loop {
                let moved = move_dir(pstart, dir);

                if moved.is_none() {
                    path_len = 0;
                    break;
                }
                let (a, b, c) = moved.unwrap();
                if pstart.x + a < 0
                    || pstart.y + b < 0
                    || pstart.y + b > width
                    || pstart.x + a > width
                {
                    path_len = 0;
                    break;
                }

                pstart = &pipes[(pstart.x + a) as usize][(pstart.y + b) as usize];
                dir = c;
                if pstart.pipe == Pipe::Start {
                    break;
                }
                if pstart.pipe == Pipe::Ground {
                    break;
                }
                path_len += 1;
            }
            path_len
        })
        .collect();

    (paths.iter().max().unwrap() + 1) / 2
}

#[allow(clippy::too_many_lines)]
fn day10part2(lines: &[String]) -> i64 {
    let width = lines[0].len() as i64;
    // Somehow build the puzzle
    let mut pipes = lines
        .iter()
        .enumerate()
        .map(|(pos, line): (usize, &String)| {
            line.chars()
                .enumerate()
                .map(|(offset, c)| match c {
                    '|' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::NS,
                    },
                    '-' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::EW,
                    },
                    'L' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::NE,
                    },
                    'J' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::NW,
                    },
                    '7' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::SW,
                    },
                    'F' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::SE,
                    },
                    'S' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::Start,
                    },
                    '.' => Cell {
                        x: pos as i64,
                        y: offset as i64,
                        pipe: Pipe::Ground,
                    },
                    _ => todo!("later"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut ss = pipes[0][0].clone();
    pipes
        .iter_mut()
        .flatten()
        .filter(|x| x.pipe == Pipe::Start)
        .for_each(|p| {
            // TODO work out what pipe the start is, NS worked for my input
            p.pipe = Pipe::NS;
            ss = p.clone();
        });

    let mut pstart = &pipes[(ss.x) as usize][(ss.y) as usize];
    //Decide which way to follow the path based on start pipe type
    let mut dir = if matches!(ss.pipe, Pipe::NS | Pipe::SE | Pipe::SW) {
        2
    } else if matches!(ss.pipe, Pipe::EW | Pipe::NW) {
        3
    } else {
        0
    };
    let mut path: Vec<&Cell> = vec![];
    path.push(pstart);
    loop {
        let moved = move_dir(pstart, dir);

        if moved.is_none() {
            break;
        }
        let (a, b, c) = moved.unwrap();
        if pstart.x + a < 0 || pstart.y + b < 0 || pstart.y + b > width || pstart.x + a > width {
            break;
        }

        pstart = &pipes[(pstart.x + a) as usize][(pstart.y + b) as usize];
        dir = c;
        if *pstart == ss {
            break;
        }
        if pstart.pipe == Pipe::Ground {
            break;
        }
        path.push(pstart);
    }
    let mut inside = 0;
    for r in &pipes {
        let mut next_sw = false;
        let mut next_nw = false;
        let mut inside_path = false;

        for b in r {
            if path.contains(&b) {
                let check = b;
                //Count number of pipe intersections
                if check.pipe == Pipe::NS || check.pipe == Pipe::Start {
                    inside_path = !inside_path;
                } else if check.pipe == Pipe::SE {
                    next_nw = true;
                } else if check.pipe == Pipe::NE {
                    next_sw = true;
                } else if check.pipe == Pipe::NW {
                    if next_nw {
                        inside_path = !inside_path;
                        next_nw = false;
                    }
                    next_sw = false;
                } else if check.pipe == Pipe::SW {
                    if next_sw {
                        next_sw = false;
                        inside_path = !inside_path;
                    }
                    next_nw = false;
                }
            } else if inside_path {
                inside += 1;
            }
        }
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day10(&lines), 8);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test2.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day10part2(&lines), 10);
    }
}
