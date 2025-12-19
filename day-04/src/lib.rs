use rayon::prelude::*;
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    fn get_roll_locations(&self) -> Vec<(usize, usize)> {
        let mut ans: Vec<(usize, usize)> = vec![];
        self.grid
            .iter()
            .enumerate()
            .for_each(|(row_index, vec_of_chars)| {
                vec_of_chars.iter().enumerate().for_each(|(col_index, ch)| {
                    if ch == &'@' {
                        ans.push((row_index, col_index));
                    }
                })
            });

        return ans;
    }

    fn get_accessable_locations(&self) -> Vec<(usize, usize)> {
        let roll_locations = self.get_roll_locations();
        let accessable_locations: Vec<(usize, usize)> = roll_locations
            .par_iter()
            .filter_map(|(x, y)| {
                if self.is_accesable(*x, *y) {
                    Some((*x, *y))
                } else {
                    None
                }
            })
            .collect();

        return accessable_locations;
    }

    fn updated_grid(&mut self, locations: Vec<(usize, usize)>) {
        for (x, y) in locations {
            self.grid[x][y] = '.';
        }
    }

    fn get_rolling_roll_counts(&mut self) -> usize {
        let mut ans = 0;

        loop {
            let locations = self.get_accessable_locations();
            if locations.len() == 0 {
                break;
            }
            ans += locations.len();
            self.updated_grid(locations);
        }

        return ans;
    }

    fn get_roll_counts(&self) -> usize {
        self.get_accessable_locations().iter().count()
    }

    fn is_accesable(&self, row: usize, col: usize) -> bool {
        let row_c = row as isize;
        let col_c = col as isize;
        let top = self.is_free(row_c - 1, col_c);
        let top_left = self.is_free(row_c - 1, col_c - 1);
        let top_right = self.is_free(row_c - 1, col_c + 1);
        let left = self.is_free(row_c, col_c - 1);
        let right = self.is_free(row_c, col_c + 1);
        let bottom = self.is_free(row_c + 1, col_c);
        let bottom_left = self.is_free(row_c + 1, col_c - 1);
        let bottom_right = self.is_free(row_c + 1, col_c + 1);
        let ans = top + top_left + top_right + left + right + bottom + bottom_left + bottom_right;

        if ans <= 4 {
            return false;
        }
        return true;
    }

    fn is_free(&self, row: isize, col: isize) -> u8 {
        if row < 0 || row >= self.grid.len() as isize {
            return 1;
        }

        if col < 0 || col >= self.grid[0].len() as isize {
            return 1;
        }

        if self.grid[row as usize][col as usize] == '@' {
            return 0;
        }

        return 1;
    }
}

pub fn get_part_one(path: String) -> usize {
    let data: Vec<Vec<char>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let grid = Grid::new(data);

    return grid.get_roll_counts();
}

pub fn get_part_two(path: String) -> usize {
    let data: Vec<Vec<char>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut grid = Grid::new(data);

    return grid.get_rolling_roll_counts();
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn read_data() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();

        let data = input.lines().map(|line| line.chars().collect()).collect();

        let mut grid = Grid::new(data);

        assert_eq!(grid.get_rolling_roll_counts(), 43);
    }
}
