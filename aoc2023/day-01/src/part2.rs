pub fn solution(input: &str) -> u32 {
    return input.lines().map(parse_line).sum::<u32>();
}

fn parse_line(line: &str) -> u32 {
    let mut idx = 0;
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &line[idx..];

        let result =
        if reduced_line.starts_with("one") {
            Some('1')
        } else if reduced_line.starts_with("two") {
            Some('2')
        } else if reduced_line.starts_with("three") {
            Some('3')
        } else if reduced_line.starts_with("four") {
            Some('4')
        } else if reduced_line.starts_with("five") {
            Some('5')
        } else if reduced_line.starts_with("six") {
            Some('6')
        } else if reduced_line.starts_with("seven") {
            Some('7')
        } else if reduced_line.starts_with("eight") {
            Some('8')
        } else if reduced_line.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced_line.chars().next();
            idx += 1;
            return result;
        };

        idx += 1;
        return result;
    });

    let mut digit_chars = line_iter.filter_map(|c| c.to_digit(10));
    let first_digit = digit_chars.next().expect("Should be a valid digit!");

    return match digit_chars.last() {
        Some(dig) => format!("{first_digit}{dig}"),
        None => format!("{first_digit}{first_digit}")
    }
    .parse::<u32>()
    .expect("Should be a valid number!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    
    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)] 
    fn test_parse_line(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, parse_line(line))
    }

    #[test]
    fn test_solution() {
        let input = 
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(281, solution(input));
    }
}
