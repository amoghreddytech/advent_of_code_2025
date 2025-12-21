use rayon::prelude::*;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn is_inside(&self, id: u64) -> bool {
        if id >= self.start && id <= self.end {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug)]
struct Id {
    id: u64,
}

impl Id {
    fn new(id: u64) -> Self {
        Self { id }
    }
}

pub fn get_part_one(path: String) -> u64 {
    let (ranges, ids) = parse(path);
    return get_spoiled(ranges, ids);
}

pub fn get_part_two(path: String) -> u64 {
    let (ranges, _) = parse(path);
    return get_fresh_ids(ranges);
}

fn parse(path: String) -> (Vec<Range>, Vec<Id>) {
    let input = std::fs::read_to_string(path).unwrap();
    let (ranges, ids) = parse_input(input);
    return (parse_ranges(ranges), parse_ids(ids));
}

fn get_fresh_ids(ranges: Vec<Range>) -> u64 {
    let merged = merge_ranges(ranges);
    let sum = merged.iter().fold(0, |mut acc, range| {
        acc += range.end - range.start + 1;
        acc
    });
    return sum;
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|range| range.start);
    ranges.into_iter().fold(Vec::new(), |mut merged, range| {
        let cur_start = range.start;
        let cur_end = range.end;

        if let Some(last_range) = merged.last_mut() {
            // case when we have something in merged
            // 3 - 5 , 10 - 14 , 12 - 18 , 16 - 20
            if cur_start <= last_range.end + 1 {
                last_range.end = last_range.end.max(cur_end);
            } else {
                merged.push(Range::new(range.start, range.end));
            }
        } else {
            // case when it's empty
            merged.push(Range::new(range.start, range.end));
        }

        merged
    })
}

fn get_spoiled(ranges: Vec<Range>, ids: Vec<Id>) -> u64 {
    ids.par_iter()
        .map(|id| {
            ranges
                .par_iter()
                .map(|range| range.is_inside(id.id))
                .any(|x| x == true)
        })
        .filter(|x| x == &true)
        .count() as u64
}

fn parse_input(input: String) -> (Vec<String>, Vec<String>) {
    let mut ranges: Vec<String> = vec![];
    let mut ids: Vec<String> = vec![];
    input.lines().for_each(|x| {
        if x.trim().len() == 0 {
        } else if x.contains('-') {
            let range = String::from(x.trim());
            ranges.push(range);
        } else {
            let id = String::from(x.trim());
            ids.push(id);
        }
    });

    return (ranges, ids);
}

fn parse_ranges(ranges: Vec<String>) -> Vec<Range> {
    ranges
        .iter()
        .map(|line| {
            let mut numbers = line.split("-");
            let start: u64 = numbers.next().unwrap().parse().unwrap();
            let end: u64 = numbers.next().unwrap().parse().unwrap();
            Range::new(start, end)
        })
        .collect()
}

fn parse_ids(ids: Vec<String>) -> Vec<Id> {
    ids.iter()
        .map(|line| {
            let line = line.trim();
            Id::new(line.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        .to_string();
        let (ranges, ids) = parse_input(input);
        let x = get_spoiled(parse_ranges(ranges.clone()), parse_ids(ids));
        assert_eq!(x, 3);
        let y = get_fresh_ids(parse_ranges(ranges));
        assert_eq!(y, 14);
    }
}
