use std::iter::zip;

#[derive(Debug)]
struct Cephalopod {
    numbers: Vec<Vec<u64>>,
    conditions: Vec<char>,
}

impl Cephalopod {
    fn new(numbers: Vec<Vec<u64>>, conditions: Vec<char>) -> Self {
        Self {
            numbers,
            conditions,
        }
    }

    fn calculate(self) -> u64 {
        let iter = zip(self.numbers, self.conditions);
        let ans: Vec<u64> = iter
            .map(|(nums, cond)| {
                if cond == '+' {
                    nums.iter().sum()
                } else {
                    nums.iter().fold(1, |mut acc, n| {
                        acc *= n;
                        acc
                    })
                }
            })
            .collect();
        ans.into_iter().sum()
    }
}

fn parse_input_part_2(input: String) -> Cephalopod {
    let mut input: Vec<String> = input.lines().map(|x| String::from(x)).collect();
    let conditions = input.pop().unwrap().replace(" ", "").chars().collect();
    let parsed: Vec<Vec<char>> = input
        .iter()
        .map(|l| l.replace(" ", "@").chars().collect())
        .collect();
    let mut new_numbers = vec![];
    for i in 0..parsed[0].len() {
        let mut temp = "".to_string();
        for j in 0..parsed.len() {
            temp.push(parsed[j][i]);
        }
        new_numbers.push(temp);
    }
    let mut final_nums: Vec<Vec<u64>> = vec![];
    let mut temp_vec: Vec<u64> = vec![];
    for num in new_numbers {
        if num.replace("@", "").len() == 0 {
            final_nums.push(temp_vec);
            temp_vec = vec![];
        } else {
            temp_vec.push(num.replace("@", "").parse().unwrap());
        }
    }

    if temp_vec.len() > 0 {
        final_nums.push(temp_vec);
    }

    Cephalopod::new(final_nums, conditions)
}

fn parse_input(input: String) -> Cephalopod {
    let mut numbers: Vec<Vec<u64>> = vec![];
    let mut conditions: Vec<char> = vec![];
    let _ = input.lines().enumerate().for_each(|(row_index, line)| {
        line.trim()
            .split_whitespace()
            .enumerate()
            .for_each(|(col_index, val)| {
                if let Ok(number) = val.parse() {
                    if row_index == 0 {
                        numbers.push(vec![number]);
                    } else {
                        numbers[col_index].push(number);
                    }
                } else {
                    conditions.push(val.chars().next().unwrap());
                }
            });
    });

    Cephalopod::new(numbers, conditions)
}

pub fn get_part_one(path: String) -> u64 {
    let input = std::fs::read_to_string(path).unwrap();
    let cephalopod = parse_input(input);
    cephalopod.calculate()
}

pub fn get_part_two(path: String) -> u64 {
    let input = std::fs::read_to_string(path).unwrap();
    let cephalopod = parse_input_part_2(input);
    cephalopod.calculate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + "
            .to_string();

        let a = parse_input_part_2(input);
        println!("{:?}", a.calculate());
    }
}
