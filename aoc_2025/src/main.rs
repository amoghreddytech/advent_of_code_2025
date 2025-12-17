fn main() {
    println!("------------- day 01 -------------------");
    let password = day_01::get_part_one("puzzle_inputs/day-01.txt".to_string());
    println!("The password to unlock the safe is {password}");
    let new_password = day_01::get_part_two("puzzle_inputs/day-01.txt".to_string());
    println!("The password to unlock the safe is {new_password}");
    println!("------------- day 02 -------------------");
    let sum_invalid_ids = day_02::get_part_one("puzzle_inputs/day-02.txt");
    println!("The sum of invalid ids = {sum_invalid_ids}");
    let sum_invalid_ids = day_02::get_part_two("puzzle_inputs/day-02.txt");
    println!("The sum of invalid ids = {sum_invalid_ids}");
}
