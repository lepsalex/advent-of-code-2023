use std::collections::HashMap;
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
    let digit_words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut total = 0;

    for line in input.split(0xA as char) {
        let mut digit_map: HashMap<usize, char> = HashMap::new();

        // find number digits and store their index to the hash map
        // as the key, with the digit char as the value
        for (idx, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                digit_map.insert(idx, char);
            }
        }

        // find word digits and store their index to the hash map,
        // for the value store the digit char instead of the word
        for (dword_idx, dword) in digit_words.iter().enumerate() {
            for (match_idx, _) in line.match_indices(dword) {
                digit_map.insert(match_idx, char::from_digit((dword_idx + 1) as u32, 10).unwrap());
            }
        }

        // if there were no digits continue
        if digit_map.is_empty() {
            continue
        }

        // Get the lowest and highest index chars
        let min = digit_map.keys().min().unwrap();
        let max = digit_map.keys().max().unwrap();

        // put them together
        let mut line_total_str = String::new();
        line_total_str.push(digit_map.get(min).unwrap().clone());
        line_total_str.push(digit_map.get(max).unwrap().clone());

        // update total
        total += line_total_str.parse::<u32>().unwrap();
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55090));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(292));
    }
}
