use std::collections::HashMap;

#[derive(Debug, Clone)]
struct UnionSet {
    boxes: Vec<JunctionBox>,
    edges: Vec<Edge>, // this will n*n
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionSet {
    fn new(boxes: Vec<JunctionBox>) -> Self {
        let n = boxes.len();
        let mut edges = vec![];
        for row in 0..n {
            for col in 0..n {
                if col >= row {
                    break;
                } else {
                    let weight = boxes[row].get_euclidian_distance(&boxes[col]);
                    edges.push(Edge::new(row, col, weight));
                }
            }
        }

        edges.sort_by_key(|f| f.weight);
        edges.reverse();

        Self {
            boxes,
            edges,
            parents: (0..n).collect(),
            sizes: vec![1; n],
        }
    }

    fn find(&mut self, mut node_id: usize) -> usize {
        // find the current root
        // [0,1,2,3,4,5,6,7,8,9,4,..] imagine we're querying 10

        let mut root = node_id;

        while root != self.parents[root] {
            root = self.parents[root];
        }

        // loop though all the nodes and change them to the parent

        while node_id != root {
            let next = self.parents[node_id];
            self.parents[node_id] = root;
            node_id = next;
        }

        root
    }

    fn union(&mut self, node_id1: usize, node_id2: usize) -> Option<(usize, usize)> {
        let root1 = self.find(node_id1);
        let root2 = self.find(node_id2);

        if root1 == root2 {
            return None;
        }

        if self.sizes[root1] >= self.sizes[root2] {
            self.sizes[root1] += self.sizes[root2];
            self.parents[root2] = root1;
        } else {
            self.sizes[root2] += self.sizes[root1];
            self.parents[root1] = root2;
        }

        if self.sizes[root1] == self.parents.len() || self.sizes[root2] == self.parents.len() {
            return Some((node_id1, node_id2));
        }

        None
    }

    fn merge_circuts_again(&mut self) -> Option<(usize, usize)> {
        for _ in 0..self.edges.len() {
            let smallest_edge = self.edges.pop().unwrap();
            if let Some(x) = self.union(smallest_edge.x, smallest_edge.y) {
                return Some((x.0, x.1));
            }
        }
        None
    }
    fn merge_circuts(&mut self, times: usize) {
        let mut index = 0;

        while index < times {
            let smallest_edge = self.edges.pop().unwrap();
            self.union(smallest_edge.x, smallest_edge.y);
            index += 1;
        }
    }

    fn process_two(&mut self) -> i64 {
        let x = self.merge_circuts_again().unwrap();
        return (self.boxes[x.0].x * self.boxes[x.1].x) as i64;
    }

    fn process(&mut self, times: usize) -> i64 {
        self.merge_circuts(times);

        let mut counts = HashMap::new();
        for i in 0..self.parents.len() {
            let root = self.find(i);
            counts.entry(root).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut values: Vec<_> = counts.values().collect();
        values.sort_by(|a, b| b.cmp(a));

        return **values.get(0).unwrap() * **values.get(1).unwrap() * **values.get(2).unwrap();
    }
}

pub fn get_part_one(path: String, times: usize) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let boxes = parse_data(&input);
    let mut unionset = UnionSet::new(boxes);
    return unionset.process(times);
}

pub fn get_part_two(path: String) -> i64 {
    let input = std::fs::read_to_string(path).unwrap();
    let boxes = parse_data(&input);
    let mut unionset = UnionSet::new(boxes);
    return unionset.process_two();
}

#[derive(Debug, Clone)]
struct Edge {
    x: usize,
    y: usize,
    weight: i64,
}

impl Edge {
    fn new(x: usize, y: usize, weight: i64) -> Self {
        Self { x, y, weight }
    }
}

#[derive(Debug, Clone)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn get_euclidian_distance(&self, other: &JunctionBox) -> i64 {
        let x = (self.x - other.x).pow(2);
        let y = (self.y - other.y).pow(2);
        let z = (self.z - other.z).pow(2);

        x + y + z
    }
}

fn parse_data(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            let mut values = line.split(',');
            let x = values.next().unwrap().parse().unwrap();
            let y = values.next().unwrap().parse().unwrap();
            let z = values.next().unwrap().parse().unwrap();
            JunctionBox::new(x, y, z)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let boxes = parse_data(input);
        let mut unionset = UnionSet::new(boxes);
        assert_eq!(unionset.process(10), 40);
        assert_eq!(unionset.process_two(), 25272);
    }
}
