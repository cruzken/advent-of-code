use std::collections::HashSet;

#[allow(dead_code)]
pub fn star_one(input: &str) -> i32 {
    let coords = build_coords(input);
    // get grid's boundaries
    let bounds = grid_bounds(&coords);
    println!(
        "x-bounds: {},{} y-bounds: {},{}",
        bounds.0, bounds.1, bounds.2, bounds.3
    );
    // filter out coords with infinite areas (on the grid boundaries)
    let infinite_points = coords
        .iter()
        .filter(|x| has_infinite(x, bounds));
    println!("{:?}", infinite_points.collect::<Vec<_>>());
    // build grid data calculating distance per point per grid coord

    0
}

fn has_infinite(point: &Point, bounds: (i32, i32, i32, i32)) -> bool {
    point.x == bounds.0 || point.x == bounds.1 || point.y == bounds.2 || point.y == bounds.3
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

fn grid_bounds(coords: &HashSet<Point>) -> (i32, i32, i32, i32) {
    let min_x = coords.iter().fold(std::i32::MAX, |lowest, point| {
        if point.x < lowest {
            return point.x;
        }
        lowest
    });
    let max_x = coords.iter().fold(std::i32::MIN, |highest, point| {
        if point.x > highest {
            return point.x;
        }
        highest
    });
    let min_y = coords.iter().fold(std::i32::MAX, |lowest, point| {
        if point.y < lowest {
            return point.y;
        }
        lowest
    });
    let max_y = coords.iter().fold(std::i32::MIN, |highest, point| {
        if point.y > highest {
            return point.y;
        }
        highest
    });
    (min_x, max_x, min_y, max_y)
}

fn build_coords(input: &str) -> HashSet<Point> {
    input.lines().map(|x| build_point(x)).collect()
}

fn build_point(input: &str) -> Point {
    let parsed = input
        .replace(",", "")
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    Point {
        x: parsed[0],
        y: parsed[1],
    }
}

#[derive(Debug, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(
                "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
            ),
            1
        )
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
