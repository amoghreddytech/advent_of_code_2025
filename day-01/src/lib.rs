struct Safe {
    dial: i32,
}

impl Safe {
    fn new(dial: i32) -> Self {
        Self { dial }
    }

    fn get_ticks_part_2(&mut self, direction: String, value: i32) -> i32 {
        let old_dial = self.dial;
        let mut ticks: i32 = 0;
        ticks += value / 100;
        let val = value % 100;

        if direction == "L" {
            self.dial = self.dial - val;
            if self.dial < 0 {
                self.dial += 100;
                if old_dial != 0 {
                    ticks += 1;
                }
            } else if self.dial == 0 {
                ticks += 1;
            }
        } else {
            self.dial += val;
            if self.dial > 99 {
                self.dial -= 100;
                ticks += 1;
            }
        }

        return ticks;
    }
    pub fn get_ticks_part_1(&mut self, direction: String, mut value: i32) -> i32 {
        let mut ticks = 0;
        value = value % 100;

        if direction == "L" {
            self.dial -= value;
            if self.dial < 0 {
                self.dial += 100;
            }
        } else {
            self.dial += value;
            if self.dial > 99 {
                self.dial -= 100;
            }
        }

        if self.dial == 0 {
            ticks += 1;
        }

        ticks
    }
}

pub fn parse_input(path: String) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn split_input(input: String) -> (String, i32) {
    if input.starts_with("L") {
        let value = input.strip_prefix("L").unwrap().parse::<i32>().unwrap();
        return ("L".to_string(), value);
    } else {
        let value = input.strip_prefix("R").unwrap().parse::<i32>().unwrap();
        return ("R".to_string(), value);
    }
}

pub fn get_part_one(path: String) -> i32 {
    let instructions = parse_input(path);
    let mut safe = Safe::new(50);
    let mut password = 0;
    for instruction in instructions {
        let (dir, value) = split_input(instruction);
        password += safe.get_ticks_part_1(dir, value);
    }

    password
}

pub fn get_part_two(path: String) -> i32 {
    let instructions = parse_input(path);
    let mut safe = Safe::new(50);
    let mut password = 0;
    for instruction in instructions {
        let (dir, value) = split_input(instruction);
        password += safe.get_ticks_part_2(dir, value);
    }

    password
}
