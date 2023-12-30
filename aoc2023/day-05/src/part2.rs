use std::ops::Range;

#[derive(Debug)]
struct Map {
    range: Range<i64>,
    delta: i64
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<Map>>) {
    let lines: Vec<&str> = input.lines().collect();
    let seeds_str = lines[0].split_once(": ").unwrap().1;
    let seed_nums = seeds_str.split(' ').map(|num| num.parse::<i64>().unwrap());
    let mut seed_starts: Vec<i64> = Vec::new();
    let mut seed_ranges: Vec<i64> = Vec::new();

    for (i, seed_num) in seed_nums.enumerate() {
        if i % 2 != 0 {
            seed_ranges.push(seed_num);
        } else {
            seed_starts.push(seed_num);
        }
    }

    let mut seeds: Vec<i64> = Vec::new();

    for i in 0..seed_starts.len() {
        let curr_seed_start = seed_starts[i];
        let curr_seed_range = seed_ranges[i];

        for s in curr_seed_start..curr_seed_start + curr_seed_range + 1 {
            seeds.push(s);
        }
    }

    let mut all_maps: Vec<Vec<Map>> = Vec::new();
    let mut this_map: Vec<Map> = Vec::default();
    for line in lines[3..].iter() {
        if line.contains(":") {
            all_maps.push(this_map);
            this_map = Vec::default();
        } else if !line.is_empty() {
            let nums: Vec<i64> = line.split(' ').map(|num| num.parse::<i64>().unwrap()).collect();
            this_map.push(Map {
                range: Range {
                    start: nums[1],
                    end: nums[1] + nums[2]
                },
                delta: nums[0] - nums[1]
            });
        }
    }
    if !this_map.is_empty() {
        all_maps.push(this_map);
    }

    return (seeds, all_maps);
}

pub fn solution(input: &str) -> i64 {
    let (seeds, all_maps) = parse_input(input);
    let mut min = i64::MAX;

    for seed in seeds {
        let mut curr_val = seed;
        for map_set in &all_maps {
            for map in map_set {
                if map.range.contains(&curr_val) {
                    curr_val += map.delta;
                    break;
                }
            }
        }
        min = min.min(curr_val);
    }

    return min;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(46, solution(input));
    }
}
