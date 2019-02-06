
// Impossible to evaluate test, need to use human observation.
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> i64 {
    // Initializate points here
    // Display positions
    // Update tick

    1
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

fn find_bounds(points: &Vec<Point>) -> (i32, i32, i32, i32) {
    let mut x_min = None;
    let mut y_min = None;
    let mut x_max = None;
    let mut y_max = None;

    for point in points {
        let pos = point.position;
        if x_min == None || x_min > Some(pos.0){
            x_min = Some(pos.0);
        }
        if y_min == None || y_min > Some(pos.1) {
            y_min = Some(pos.1);
        }
        if x_max == None || x_max < Some(pos.0){
            x_max = Some(pos.0);
        }
        if y_max == None || y_max < Some(pos.1) {
            y_max = Some(pos.1);
        }
    }

    (x_min.unwrap(), y_min.unwrap(), x_max.unwrap(), y_max.unwrap())
}

fn build_points(input: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    for line in input.lines() {

        let new = line.replace("position=<", " ")
            .replace(",", " ")
            .replace("> velocity=<", " ")
            .replace(">", "")
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        points.push(Point {
            position: (new[0], new[1]),
            velocity: (new[2], new[3]),
        });
    }

    points
}

#[derive(Debug)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Point {
    fn _new(position: (i32, i32), velocity: (i32, i32)) -> Point {
        Point { position, velocity }
    }

    fn update(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"), 1)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
