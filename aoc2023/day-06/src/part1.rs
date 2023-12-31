struct Race {
    time: i64,
    best_distance: i64
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<i64> = lines[0]
        .split_once(": ")
        .unwrap().1
        .split_whitespace()
        .map(|time| time.parse::<i64>().unwrap())
        .collect();
    let distances: Vec<i64> = lines[1]
        .split_once(": ")
        .unwrap().1
        .split_whitespace()
        .map(|distance| distance.parse::<i64>().unwrap())
        .collect();
    let mut races: Vec<Race> = Vec::new();
    for i in 0..times.len() {
        races.push(Race { time: times[i], best_distance: distances[i] });
    }

    return races;
}

pub fn solution(input: &str) -> i64 {
    let races = parse_input(input);
    let mut sol = 1;

    for race in races {
        let a = -1f64;
        let b = race.time as f64;
        let c = (-race.best_distance) as f64;

        let mut first_root = ((-b + (b * b - 4. * a * c).sqrt()) / (2. * a)).ceil();
        let mut second_root = ((-b - (b * b - 4. * a * c).sqrt()) / (2. * a)).floor();

        if (first_root * (b - first_root)) == race.best_distance as f64 {
            first_root += 1.;
        }

        if (second_root * (b - second_root)) == race.best_distance as f64 {
            second_root -= 1.;
        }

        sol *= (second_root - first_root) as i64 + 1;
    }

    return sol;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(288, solution(input));
    }
}