#[derive(Debug, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32
}

impl Round {
    fn is_valid(&self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }
}

#[derive(Debug, Default)]
struct Game {
    id: u32,
    rounds: Vec<Round>
}

impl Game {
    fn is_valid(&self) -> bool {
        return self.rounds.iter().all(|round| round.is_valid());
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    let games = input.lines();
    let mut games_list = Vec::new();
    for game in games {
        let mut g = Game::default();
        let (game_pref, rounds) = game.split_once(": ").unwrap();
        let (_, game_id) = game_pref.split_once(" ").unwrap();
        let rounds = rounds.split("; ").collect::<Vec<_>>();
        let mut round_list = Vec::new();
        for round in rounds {
            let cubes = round.split(", ").collect::<Vec<_>>();
            let mut r = Round::default();
            for cube in cubes {
                let (count, color) = cube.split_once(" ").unwrap();
                let count = count.parse::<u32>().expect("Should be a valid number!");
                match color {
                    "red" => r.red = count,
                    "green" => r.green = count,
                    "blue" => r.blue = count,
                    _ => panic!("Found an invalid color!")
                }
            }
            round_list.push(r);
        }
        g.id = game_id.parse::<u32>().expect("Should be a valid number!");
        g.rounds = round_list;
        games_list.push(g);
    }

    return games_list;
}


pub fn solution(input: &str) -> u32 {
    let parsed_input = parse_input(input);
    let id_sum = parsed_input.iter().fold(0, |acc, game| if game.is_valid() { acc + game.id } else { acc });
    
    return id_sum;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, solution(input));
    }
}