use std::{time::Instant, ops::ControlFlow, collections::HashMap};
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
    let mut p: Vec<_> = (0..lines[0].len())
                .map(|i| lines.iter().map(|inner| inner.chars().nth(i).unwrap()).collect::<Vec<_>>())
                .collect();
    for i in 0 ..p.len(){
        let mut last_first_empty: i32 = -1;
        for ci in 0 .. p[i].len() {
            if p[i][ci] == '.' && last_first_empty == -1 {
                last_first_empty = ci as i32;
            }
            if p[i][ci] == '#' {
                last_first_empty = -1;
            }
            if p[i][ci] == 'O' && last_first_empty != -1 {
                p[i][ci] = '.';
                p[i][last_first_empty as usize] = 'O';
                last_first_empty += 1;
            }
        };
    }
    let p2: Vec<_> = (0..p[0].len())
                .map(|i| p.iter().map(|inner| inner[i]).collect::<Vec<_>>())
                .collect();

    p2.iter().enumerate().map(|(i, l)| (l.iter().filter(|x| **x == 'O').count() * (lines.len()-i)) as i64).sum()
}

fn day14part2(lines: &[String]) -> i64 {
    // translate once so north is to the left and all shifts are done to the left/right
    let mut p: Vec<Vec<char>> = (0..lines[0].len())
    .map(|i| lines.iter().map(|inner| inner.chars().nth(i).unwrap()).collect::<Vec<_>>())
    .collect();
    let mut map:HashMap<Vec<Vec<char>>, usize> = HashMap::default();
    for x in 0 .. 1_000_000_000 {
        cycle(&mut p);
        if map.contains_key(&p) {
            //already seen check index
            let last = map.get(&p).unwrap();
            let diff = x - last;
            let todo = (1_000_000_000 - last) % diff -1;
            for _ in 0 .. todo {
                cycle(&mut p);
            }
            break;
        } 
        map.insert(p.clone(), x);
    }

    p = transpose(&p);
   
    p.iter().enumerate().map(|(i, l)| (l.iter().filter(|x| **x == 'O').count() * (lines.len()-i)) as i64).sum()
}

fn cycle(p: &mut Vec<Vec<char>>) {
    for y in 0 .. 4 {
        match y { 
            0 => move_rocks_left(p),
            1 => {*p = transpose(p); move_rocks_left(p)},
            2 | 3 => {*p = transpose(p); move_rocks_right(p)}, 
            _ => ()
        }
    }
    *p = transpose(p);

    
}

fn transpose(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let p2: Vec<_> = (0..p[0].len())
                .map(|i| p.iter().map(|inner| inner[i]).collect::<Vec<_>>())
                .collect();
    p2
}

fn move_rocks_left(p: &mut Vec<Vec<char>>) {
    for i in 0 ..p.len(){
        let mut last_first_empty: i32 = -1;
        for ci in 0 .. p[i].len() {
            if p[i][ci] == '.' && last_first_empty == -1 {
                last_first_empty = ci as i32;
            }
            if p[i][ci] == '#' {
                last_first_empty = -1;
            }
            if p[i][ci] == 'O' && last_first_empty != -1 {
                p[i][ci] = '.';
                p[i][last_first_empty as usize] = 'O';
                last_first_empty += 1;
                if p[i][last_first_empty as usize] != '.' {
                    last_first_empty = -1;
                }
            }
        };
    }
}

fn move_rocks_right(p: &mut Vec<Vec<char>>) {
    for i in (0 ..p.len()).rev() {
        let mut last_first_empty: i32 = -1;
        for ci in (0 .. p[i].len()).rev() {
            //println!("{i} {ci} {last_first_empty}");
            if p[i][ci] == '.' && last_first_empty == -1 {
                last_first_empty = ci as i32;
            }
            if p[i][ci] == '#' {
                last_first_empty = -1;
            }
            if p[i][ci] == 'O' && last_first_empty != -1 {
                p[i][ci] = '.';
                p[i][last_first_empty as usize] = 'O';
                last_first_empty -= 1;
                if p[i][last_first_empty as usize] != '.' {
                    last_first_empty = -1;
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day14(&lines), 136);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day14part2(&lines), 64);
    }
}
