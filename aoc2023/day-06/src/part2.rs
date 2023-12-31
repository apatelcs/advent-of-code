struct Race {
    time: i64,
    best_distance: i64
}

fn parse_input(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();
    let time = lines[0]
        .split_once(": ")
        .unwrap().1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let best_distance = lines[1]
        .split_once(": ")
        .unwrap().1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    return Race { time, best_distance };
}

pub fn solution(input: &str) -> i64 {
    let race = parse_input(input);

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

    return (second_root - first_root) as i64 + 1;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(71503, solution(input));
    }
}