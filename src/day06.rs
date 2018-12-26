#[allow(dead_code)]
pub fn star_one(input: &str) -> i32 {
    let p1 = Point {x: 1, y: 1};
    let p2 = Point {x: 3, y: 5};
    p1.distance(&p2)
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"), 1)
    }

#[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
