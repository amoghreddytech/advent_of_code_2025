use rayon::prelude::*;

#[derive(Clone, Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn check_if_invalid(num: u64) -> bool {
        let number = num.to_string();
        if number.len() % 2 != 0 {
            return false;
        } else {
            let middle_index = number.len() / 2;
            let (first, last) = number.split_at(middle_index);
            return first == last;
        }
    }

    fn check_if_invalid_part_2(num: u64) -> bool {
        let number = num.to_string();
        let chars: Vec<char> = number.chars().collect();
        let len = chars.len();
        // 1010

        for pattern_len in 1..(len / 2 + 1) {
            if len % pattern_len == 0 {
                let mut repeating = true;
                for i in pattern_len..len {
                    if chars[i] != chars[i % pattern_len] {
                        repeating = false;
                        break;
                    }
                }

                if repeating == true {
                    return true;
                }
            }
        }

        false
    }

    fn sum_invalid_ids(&self) -> u64 {
        let range: std::ops::Range<u64> = self.start..self.end + 1;
        let values: u64 = range
            .into_iter()
            .filter(|val| Range::check_if_invalid(*val))
            .sum();
        values
    }

    fn sum_invalid_ids_part_2(&self) -> u64 {
        let range: std::ops::Range<u64> = self.start..self.end + 1;
        let values: u64 = range
            .into_iter()
            .filter(|val| Range::check_if_invalid_part_2(*val))
            .sum();
        values
    }
}

fn parse_input(path: &str) -> Vec<Range> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(",")
        .map(|mut single_range| {
            single_range = single_range.trim_end();
            let mut splitter = single_range.split("-");
            let start = splitter.next().unwrap().parse().unwrap();
            let end = splitter.next().unwrap().parse().unwrap();
            Range::new(start, end)
        })
        .collect()
}

pub fn get_part_one(path: &str) -> u64 {
    let ranges = parse_input(path);
    ranges.into_par_iter().map(|x| x.sum_invalid_ids()).sum()
}

pub fn get_part_two(path: &str) -> u64 {
    let ranges = parse_input(path);
    ranges
        .into_par_iter()
        .map(|x| x.sum_invalid_ids_part_2())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn check_if_invalid_part_2() {
        let x = Range::check_if_invalid_part_2(123123123);
        assert_eq!(x, true);
        let x = Range::check_if_invalid_part_2(1010);
        assert_eq!(x, true);
        let x = Range::check_if_invalid_part_2(1188511885);
        assert_eq!(x, true);
        let x = Range::check_if_invalid_part_2(22222);
        assert_eq!(x, true);
        let x = Range::check_if_invalid_part_2(446446);
        assert_eq!(x, true);
        let x = Range::check_if_invalid_part_2(2121212121);
        assert_eq!(x, true);
    }

    #[test]
    fn check_if_invalid() {
        let x = Range::check_if_invalid(11);
        assert_eq!(x, true);
        let x = Range::check_if_invalid(22);
        assert_eq!(x, true);
        let x = Range::check_if_invalid(99);
        assert_eq!(x, true);
        let x = Range::check_if_invalid(1010);
        assert_eq!(x, true);
        let x = Range::check_if_invalid(1188511885);
        assert_eq!(x, true);
        let x = Range::check_if_invalid(22222);
        assert_eq!(x, false);
        let x = Range::check_if_invalid(446446);
        assert_eq!(x, true);
        let x = Range::check_if_invalid(446448);
        assert_eq!(x, false);
    }

    #[test]
    fn test_invalid_ids() {
        let range = Range::new(11, 22);
        let x = range.sum_invalid_ids();
        assert_eq!(x, 33);

        let range = Range::new(38593856, 38593862);
        let x = range.sum_invalid_ids();
        assert_eq!(x, 38593859);
    }
}
