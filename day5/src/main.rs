use std::time::Instant;
fn main() {
    let file = include_str!("../input.data");
    let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
    {
        let now = Instant::now();
        let total_part1 = day5(&lines);
        println!("Part 1 {total_part1}");
        println!("Done in: {:.2?}", now.elapsed());
    }

    {
        let now = Instant::now();
        let total_part2 = day5part2(&lines);
        println!("Part 2 {total_part2}");
        println!("Done in: {:.2?}", now.elapsed());
    }
}

enum Maps {
    Seed2Soil,
    Soil2Fert,
    Fert2Water,
    Water2Light,
    Light2Temp,
    Temp2Humid,
    Humid2Loc,
}

#[derive(Debug, Clone)]
struct Range {
    source: u64,
    dest: u64,
    range: u64,
}

fn line_2_range(line: &str) -> Range {
    let l: Vec<_> = line.split(' ').collect();
    Range {
        source: l[1].parse::<u64>().unwrap(),
        dest: l[0].parse::<u64>().unwrap(),
        range: l[2].parse::<u64>().unwrap(),
    }
}

fn day5(lines: &[String]) -> u64 {
    let seed_numbers: Vec<u64> = lines[0]["seeds: ".len()..]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let LoadMapsResult(
        seed2soil,
        soil2fert,
        fert2water,
        water2light,
        light2temp,
        temp2humid,
        humid2location,
    ) = load_maps(lines);
    *seed_numbers
        .iter()
        .map(|seed| {
            let soil = conv_number(&seed2soil, *seed);
            let fert = conv_number(&soil2fert, soil);
            let water = conv_number(&fert2water, fert);
            let l = conv_number(&water2light, water);
            let t = conv_number(&light2temp, l);
            let h = conv_number(&temp2humid, t);
            conv_number(&humid2location, h)
        })
        .collect::<Vec<_>>()
        .iter()
        .min()
        .unwrap()
}

struct LoadMapsResult(
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
    Vec<Range>,
);

fn load_maps(lines: &[String]) -> LoadMapsResult {
    let mut seed2soil: Vec<Range> = vec![];
    let mut soil2fert: Vec<Range> = vec![];
    let mut fert2water: Vec<Range> = vec![];
    let mut water2light: Vec<Range> = vec![];
    let mut light2temp: Vec<Range> = vec![];
    let mut temp2humid: Vec<Range> = vec![];
    let mut humid2location: Vec<Range> = vec![];
    let mut map: Option<Maps> = None;
    for line in &lines[1..] {
        if line == "seed-to-soil map:" {
            map = Some(Maps::Seed2Soil);
        } else if line == "soil-to-fertilizer map:" {
            map = Some(Maps::Soil2Fert);
        } else if line == "fertilizer-to-water map:" {
            map = Some(Maps::Fert2Water);
        } else if line == "water-to-light map:" {
            map = Some(Maps::Water2Light);
        } else if line == "light-to-temperature map:" {
            map = Some(Maps::Light2Temp);
        } else if line == "temperature-to-humidity map:" {
            map = Some(Maps::Temp2Humid);
        } else if line == "humidity-to-location map:" {
            map = Some(Maps::Humid2Loc);
        } else if line.is_empty() {
            continue;
        } else if let Some(ref m) = map {
            match m {
                Maps::Seed2Soil => seed2soil.push(line_2_range(line)),
                Maps::Soil2Fert => soil2fert.push(line_2_range(line)),
                Maps::Fert2Water => fert2water.push(line_2_range(line)),
                Maps::Water2Light => water2light.push(line_2_range(line)),
                Maps::Light2Temp => light2temp.push(line_2_range(line)),
                Maps::Temp2Humid => temp2humid.push(line_2_range(line)),
                Maps::Humid2Loc => humid2location.push(line_2_range(line)),
            }
        }
    }
    LoadMapsResult(
        seed2soil,
        soil2fert,
        fert2water,
        water2light,
        light2temp,
        temp2humid,
        humid2location,
    )
}

fn day5part2(lines: &[String]) -> u64 {
    let seed_numbers = lines[0]["seeds: ".len()..]
        .split(' ')
        .collect::<Vec<_>>()
        .chunks(2)
        .filter(|x| !x.is_empty())
        .map(|s| {
            (
                s[0].parse::<u64>().unwrap(),
                s[0].parse::<u64>().unwrap() + s[1].parse::<u64>().unwrap() - 1,
            )
        })
        .collect();
    seed2location(lines, &seed_numbers)
}

fn seed2location(lines: &[String], seed_numbers: &Vec<(u64, u64)>) -> u64 {
    let LoadMapsResult(
        seed2soil,
        soil2fert,
        fert2water,
        water2light,
        light2temp,
        temp2humid,
        humid2location,
    ) = load_maps(lines);
    for location in 0..u64::MAX {
        let h = conv_number_rev(&humid2location, location);
        let t = conv_number_rev(&temp2humid, h);
        let li = conv_number_rev(&light2temp, t);
        let water = conv_number_rev(&water2light, li);
        let fert = conv_number_rev(&fert2water, water);
        let soil = conv_number_rev(&soil2fert, fert);
        let seed = conv_number_rev(&seed2soil, soil);
        for s in seed_numbers {
            if seed >= s.0 && seed <= s.1 {
                return location;
            }
        }
    }
    0
}

fn conv_number(map: &[Range], num: u64) -> u64 {
    match map
        .iter()
        .find(|x| num >= x.source && num <= x.source + x.range)
    {
        Some(v) => (num - v.source) + v.dest,
        None => num,
    }
}

fn conv_number_rev(map: &[Range], num: u64) -> u64 {
    for range in map {
        if num >= range.dest && num <= (range.dest + range.range - 1) {
            return (num - range.dest) + range.source;
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day5(&lines), 35);
    }

    #[test]
    fn part2() {
        let file = include_str!("../test.data");
        let lines: Vec<String> = file.lines().map(std::string::ToString::to_string).collect();
        assert_eq!(day5part2(&lines), 46);
    }
}
