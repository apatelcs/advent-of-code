pub fn solution(input: &str) -> u32 {
    let output = input.lines().map(|line| {
        let mut digit_chars = line.chars().filter_map(|c| c.to_digit(10));
        let first_digit = digit_chars.next().expect("Should be a valid digit!");

        match digit_chars.last() {
            Some(dig) => format!("{first_digit}{dig}"),
            None => format!("{first_digit}{first_digit}")
        }
        .parse::<u32>()
        .expect("Should be a valid number!")
    })
    .sum::<u32>();

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = 
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(142, solution(input));
    }
}
