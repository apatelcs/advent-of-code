use std::collections::HashSet;

struct PartNumber {
    val: u32,
    surrounding: HashSet<(i32, i32)>
}

impl PartNumber {
    fn new(row: i32, col: i32, c: char) -> Self {
        let surrounding = HashSet::from([
            (row - 1, col - 1), (row, col - 1), (row + 1, col - 1), // left coords
            (row - 1, col), (row + 1, col),                         // above and below coords
            (row - 1, col + 1), (row, col + 1), (row + 1, col + 1)  // right coords
        ]);

        return Self { val: (c as u32 - '0' as u32), surrounding };
    }

    fn extend_by_digit(&mut self, row: i32, col: i32, c: char) {
        self.val = self.val * 10 + (c as u32 - '0' as u32);
        self.surrounding.extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }
}

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>, Vec<PartNumber>) {
    let lines = input.lines();
    let mut curr_part_number: Option<PartNumber> = None;
    let mut syms_coords: HashSet<(i32, i32)> = HashSet::default();
    let mut gear_coords: HashSet<(i32, i32)> = HashSet::default(); 
    let mut part_numbers: Vec<PartNumber> = Vec::default();

    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                match curr_part_number {
                    Some(ref mut num) => num.extend_by_digit(row as i32, col as i32, c),
                    None => curr_part_number = Some(PartNumber::new(row as i32, col as i32, c))
                }
            } else {
                if let Some(num) = curr_part_number.take() {
                    part_numbers.push(num);
                }
                if c != '.' {
                    syms_coords.insert((row as i32, col as i32));
                    if c == '*' { gear_coords.insert((row as i32, col as i32)); }
                }
            }
        }
    }

    return (syms_coords, gear_coords, part_numbers);
}

pub fn solution(input: &str) -> u32 {
    let (syms_coords, gear_coords, part_numbers) = parse_input(input);
    let mut total = 0;

    let confirmed_part_numbers: Vec<&PartNumber> = part_numbers
        .iter()
        .filter(|part_num| { part_num.surrounding.intersection(&syms_coords).next().is_some() }).collect();

    for gear in &gear_coords {
        let mut matches = Vec::new();
        for num in &confirmed_part_numbers {
            if num.surrounding.contains(gear) {
                matches.push(num.val);
            }
        }
        if matches.len() == 2 { total += matches[0] * matches[1]; }
    }

    return total;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        
        assert_eq!(467835, solution(input));
    }
}