// given a vertex say i have 1
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Puzzle {
    vertices: Vec<Vertex>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Outside,
    Boundry,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let vertices: Vec<Vertex> = input
            .lines()
            .map(|line| {
                let collection: Vec<_> =
                    line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
                let x = collection[1];
                let y = collection[0];
                let vertex = Vertex::new(x, y);
                vertex
            })
            .collect();

        Self { vertices }
    }

    fn solve_part_two(&self) -> i64 {
        // This function is long and annoying
        // basically its grid compaction -> prefix_sums -> rectangle_check -> area

        let x_coordinates: Vec<i64> = self.vertices.iter().map(|v| v.x).collect();
        let y_cooridnates: Vec<i64> = self.vertices.iter().map(|v| v.y).collect();

        let mut compact_x_coords = x_coordinates.clone();
        compact_x_coords.sort_unstable();
        compact_x_coords.dedup();

        let mut compact_y_coords = y_cooridnates.clone();
        compact_y_coords.sort_unstable();
        compact_y_coords.dedup();

        let x_coord_map =
            compact_x_coords
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (index, x)| {
                    acc.insert(x, index);
                    acc
                });

        let y_coord_map =
            compact_y_coords
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (index, y)| {
                    acc.insert(y, index);
                    acc
                });

        let new_rows = compact_x_coords.len();
        let new_cols = compact_y_coords.len();

        let mut grid = vec![vec![Tile::Outside; new_cols]; new_rows];

        for (v1, v2) in self.vertices.iter().circular_tuple_windows() {
            let x0 = x_coord_map[&v1.x];
            let y0 = y_coord_map[&v1.y];
            let x1 = x_coord_map[&v2.x];
            let y1 = y_coord_map[&v2.y];

            if x0 == x1 {
                for c in y0.min(y1)..=y0.max(y1) {
                    grid[x0][c] = Tile::Boundry;
                }
            }

            if y0 == y1 {
                for r in x0.min(x1)..=x0.max(x1) {
                    grid[r][y0] = Tile::Boundry;
                }
            }
        }

        for row in &mut grid {
            let mut left_index = 0;
            let mut right_index = new_cols - 1;

            for r in 0..new_cols {
                if row[r] == Tile::Boundry {
                    left_index = r;
                    break;
                }
            }

            for r in (0..new_cols).rev() {
                if row[r] == Tile::Boundry {
                    right_index = r;
                    break;
                }
            }

            for x in left_index..right_index {
                row[x] = Tile::Boundry;
            }
        }

        let mut prefix_mat = vec![vec![0; new_cols + 1]; new_rows + 1];

        for (r, row) in grid.iter().enumerate() {
            let mut prefix = 0;
            for (c, val) in row.iter().enumerate() {
                if val == &Tile::Outside {
                    prefix += 1;
                }
                prefix_mat[r + 1][c + 1] = prefix + prefix_mat[r][c + 1];
            }
        }

        let mut max_area = 0;

        for (v1, v2) in self.vertices.iter().tuple_combinations() {
            let x1 = v1.x;
            let x2 = v2.x;
            let y1 = v1.y;
            let y2 = v2.y;
            // projections
            let p_x1 = x_coord_map[&x1];
            let p_x2 = x_coord_map[&x2];
            let p_y1 = y_coord_map[&y1];
            let p_y2 = y_coord_map[&y2];

            // Now I want to make use my prefix sum for this rect is 0
            // [0, 0, 0, 0, 0]
            // [0, 1, x, 1, 1]
            // [0, 1, 1, 1, 1]
            // [0, 1, 1, 1, x]
            let complete_rectange = prefix_mat[p_x1.max(p_x2) + 1][p_y1.max(p_y2) + 1];
            let left_side = prefix_mat[p_x1.max(p_x2) + 1][p_y1.min(p_y2)];
            let top_side = prefix_mat[p_x1.min(p_x2)][p_y1.max(p_y2) + 1];
            let additional_part = prefix_mat[p_x1.min(p_x2)][p_y1.min(p_y2)];

            let invalid_tiles = complete_rectange - left_side - top_side + additional_part;

            if invalid_tiles == 0 {
                let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
                max_area = max_area.max(area);
            }
        }

        max_area as i64
    }

    fn get_max_area(&self) -> i64 {
        let mut max_area = 0;
        for first in 0..self.vertices.len() {
            for second in 0..self.vertices.len() {
                if second >= first {
                    break;
                }

                let temp_area = &self.vertices[first].get_area(&self.vertices[second]);
                max_area = max_area.max(*temp_area);
            }
        }

        max_area
    }
}

pub fn get_part_two(path: String) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let puzzle = Puzzle::new(&input);
    puzzle.solve_part_two()
}

pub fn get_part_one(path: String) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let puzzle = Puzzle::new(&input);
    puzzle.get_max_area()
}

#[derive(Debug, Clone)]
struct Vertex {
    x: i64,
    y: i64,
}

impl Vertex {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn get_area(&self, other: &Vertex) -> i64 {
        if self.x == other.x && self.y == other.y {
            return 1;
        } else if self.x == other.x {
            return (self.y - other.y).abs() + 1;
        } else if self.y == other.y {
            return (self.x - other.x).abs() + 1;
        } else {
            let x_diff = (self.x - other.x).abs() + 1;
            let y_diff = (self.y - other.y).abs() + 1;
            return x_diff * y_diff;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let puzzle = Puzzle::new(input);
        assert_eq!(puzzle.get_max_area(), 50);
        assert_eq!(puzzle.solve_part_two(), 24);
    }
}
