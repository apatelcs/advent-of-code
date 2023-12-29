struct Card {
    winning_nums: Vec<u32>,
    nums: Vec<u32>
}

impl Card {
    fn how_many_matches(&self) -> i32 {
        return self.winning_nums.iter().fold(0, |acc, num| {
            if self.nums.contains(&num) {
                acc + 1
            } else {
                acc
            }
        });
    }
}

fn parse_input(input:&str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let (_, rest) = line.split_once(": ").unwrap();
        let (winning, given) = rest.split_once("| ").unwrap();
        let winning_nums: Vec<u32> = winning
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Should be a valid number!"))
            .collect();
        let nums: Vec<u32> = given
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Should be a valid number!"))
            .collect();
        cards.push(Card { winning_nums, nums })
    }

    return cards;
}

pub fn solution(input: &str) -> u32 {
    let cards = parse_input(input);
    let mut instances = vec![1usize; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let matches = card.how_many_matches();

        for ind in i + 1..(i + 1 + matches as usize) as usize {
            instances[ind] += instances[i];
        }
    }

    return instances.iter().sum::<usize>() as u32;
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

        assert_eq!(30, solution(input));
    }
}

