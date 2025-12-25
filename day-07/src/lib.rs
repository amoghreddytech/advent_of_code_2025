use std::collections::HashSet;

fn parse_part_two(input: &str) -> usize {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut beam_counts = vec![0; first_line.len()];

    for (index, ch) in first_line.chars().enumerate() {
        if ch == 'S' {
            beam_counts[index] = 1;
            break;
        }
    }

    lines
        .fold(beam_counts, |acc, line| {
            let mut new_beam_counts = vec![0; acc.len()];
            line.chars().enumerate().for_each(|(i, ch)| match ch {
                '^' => {
                    new_beam_counts[i - 1] += acc[i];
                    new_beam_counts[i + 1] += acc[i];
                }
                _ => {
                    new_beam_counts[i] += acc[i];
                }
            });
            new_beam_counts
        })
        .iter()
        .sum()

    // First run

    // for (_, line) in lines {
    //     let mut new_beam_counts = vec![0; beam_counts.len()];
    //     let mut i = 0;
    //     let line_chars: Vec<char> = line.chars().collect();

    //     while i < line_chars.len() {
    //         let ch = line_chars[i];
    //         if ch == '^' {
    //             new_beam_counts[i - 1] += beam_counts[i];
    //             new_beam_counts[i + 1] += beam_counts[i];
    //             i += 1;
    //         } else {
    //             new_beam_counts[i] += beam_counts[i];
    //             i += 1;
    //         }
    //     }

    //     beam_counts = new_beam_counts;
    // }

    // beam_counts.iter().sum()
}

pub fn get_part_two(path: String) -> usize {
    let input = std::fs::read_to_string(path).unwrap();
    parse_part_two(&input)
}

pub fn get_part_one(path: String) -> usize {
    let input = std::fs::read_to_string(path).unwrap();
    parse_part_one(&input)
}

fn parse_part_one(input: &str) -> usize {
    // first we need to get the location of the beam
    let mut iter = input.lines().enumerate();
    let beam_position = iter.next().unwrap().1.find(|x| x == 'S').unwrap();
    let mut splits: HashSet<(usize, usize)> = HashSet::new();
    // The first thing should be the positions I need to check
    // the second thing
    iter.fold(HashSet::from([beam_position]), |acc, (y_index, line)| {
        let mut new_positions = HashSet::new();
        for x_index in acc {
            // We're checking if the position
            // underneath the beam is a splitter
            if line.as_bytes()[x_index] == '^' as u8 {
                splits.insert((x_index, y_index));
                new_positions.insert(x_index - 1);
                new_positions.insert(x_index + 1);
            } else if line.as_bytes()[x_index] == '.' as u8 {
                new_positions.insert(x_index);
            }
        }

        new_positions
    });

    splits.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(21, parse_part_one(input));
        assert_eq!(40, parse_part_two(input));
    }
}
