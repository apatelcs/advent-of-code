fn parse_input(input:&str) -> i32 {
    let mut winnings: Vec<i32> = Vec::new();
    let base: i32 = 2;

    for line in input.lines() {
        let (_, rest) = line.split_once(": ").unwrap();
        let (winning, given) = rest.split_once("| ").unwrap();
        let winning_nums = winning
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Should be a valid number!"));
        let given_nums: Vec<u32> = given
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Should be a valid number!"))
            .collect();
        let num_winning = winning_nums.fold(0, |acc, num| {
            if given_nums.contains(&num) {
                acc + 1
            } else {
                acc
            }
        });
        
        if num_winning > 0 { winnings.push(base.pow(num_winning - 1)) }
    }

    return winnings.iter().sum::<i32>();
}

pub fn solution(input: &str) -> u32 {
    return parse_input(input) as u32
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, solution(input));
    }
}
