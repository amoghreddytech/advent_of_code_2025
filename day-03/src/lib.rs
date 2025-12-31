use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Battery {
    bank: String,
}

impl Battery {
    fn new(bank: String) -> Self {
        Self { bank }
    }

    fn get_12_digit_max(&self) -> u64 {
        let chars: Vec<u64> = self
            .bank
            .chars()
            .map(|ch| ch.to_digit(10).expect("It failed here maybe") as u64)
            .collect();

        let len = chars.len();

        let mut number_to_remove = len - 12;

        let mut stack: Vec<u64> = vec![];

        for ch in chars.iter() {
            while !stack.is_empty() && number_to_remove > 0 && ch > stack.last().unwrap() {
                stack.pop();
                number_to_remove -= 1;
            }

            stack.push(*ch);
        }

        let _ = stack.split_off(12);

        let num = stack
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        num
    }

    #[allow(unused_assignments)]
    fn find_max_digits(&self) -> u32 {
        let mut first_digit: u32 = 0;
        let mut second_digit: u32 = 0;
        let mut index: usize = 0;
        let mut chars: Vec<u32> = self
            .bank
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect();

        for (i, &val) in chars.iter().enumerate() {
            if val > first_digit {
                first_digit = val;
                index = i;
            }
        }

        if index == chars.len() - 1 {
            second_digit = first_digit;
            chars.pop();
            first_digit = *chars.iter().max().unwrap();
            return first_digit * 10 + second_digit;
        } else {
            let second_vec = chars.split_off(index + 1);
            second_digit = *second_vec.iter().max().unwrap();
            return first_digit * 10 + second_digit;
        }
    }
}

fn parse_input(path: String) -> Vec<Battery> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| Battery::new(x.to_string()))
        .collect()
}

pub fn get_part_one(path: String) -> u32 {
    let batteries = parse_input(path);
    batteries.par_iter().map(|x| x.find_max_digits()).sum()
}

pub fn get_part_two(path: String) -> u64 {
    let batteries = parse_input(path);
    batteries.par_iter().map(|x| x.get_12_digit_max()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_digit() {
        let bank = Battery::new("987654321111111".to_string());
        let i = bank.find_max_digits();
        assert_eq!(i, 98);
        let bank = Battery::new("323348".to_string());
        let i = bank.find_max_digits();
        assert_eq!(i, 48);
    }

    #[test]
    fn test_max_digit_again() {
        let bank = Battery::new("987654321111111".to_string());
        let i = bank.get_12_digit_max();
        assert_eq!(i, 987654321111);

        let bank = Battery::new("811111111111119".to_string());
        let i = bank.get_12_digit_max();
        assert_eq!(i, 811111111119);

        let bank = Battery::new("234234234234278".to_string());
        let i = bank.get_12_digit_max();
        assert_eq!(i, 434234234278);
    }
}
