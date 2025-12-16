use day_01::{get_part_one, get_part_two};

fn main() {
    println!("------------- day 01 -------------------");
    let password = get_part_one("puzzle_inputs/day-01.txt".to_string());
    println!("The password to unlock the safe is {password}");
    let new_password = get_part_two("puzzle_inputs/day-01.txt".to_string());
    println!("The password to unlock the safe is {new_password}");
    println!("------------- day 02 -------------------");
}
