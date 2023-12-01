advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {

    let mut total = 0;

    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for (idx, char) in input.chars().enumerate() {
        if char.is_digit(10) && first == None {
            first = Some(char)
        }

        if char.is_digit(10) {
            last = Some(char)
        }

        if char == 0xA as char || idx == (input.len() - 1) {
            // combine first/last for total (as string)
            let mut line_total_str = String::new();
            line_total_str.push(first.unwrap());
            line_total_str.push(last.unwrap());

            // update total
            total += line_total_str.parse::<u32>().unwrap();

            // reset
            first = None;
            last = None;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
