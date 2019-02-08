use std::cmp::{max, min};
use std::collections::HashSet;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> String {
    get_message_in_time(input).0
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> u32 {
    get_message_in_time(input).1
}

fn get_message_in_time(input: &str) -> (String, u32) {
    let mut points = build_points(&input);
    let mut output = String::new();
    let mut tick = 0;

    loop {
        let (x_min, _, x_max, _) = find_bounds(&points);
        let last_width = x_max - x_min;

        let positions = points
            .iter()
            .map(|x| x.position)
            .collect::<HashSet<(i32, i32)>>();

        for point in points.iter_mut() {
            point.update();
        }

        let (x_min, _, x_max, _) = find_bounds(&points);

        if x_max - x_min > last_width {
            for point in points.iter_mut() {
                point.rev();
            }

            // Display positions
            let (x_min, y_min, x_max, y_max) = find_bounds(&points);
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    if positions.contains(&(x, y)) {
                        output.push('#');
                    } else {
                        output.push('.');
                    }
                }
                output.push_str("\n");
            }
            break;
        }
        tick += 1;
    }
    (output, tick)
}

fn find_bounds(points: &Vec<Point>) -> (i32, i32, i32, i32) {
    let mut x_min = std::i32::MAX;
    let mut y_min = std::i32::MAX;
    let mut x_max = std::i32::MIN;
    let mut y_max = std::i32::MIN;

    for point in points {
        let pos = point.position;
        x_min = min(x_min, pos.0);
        x_max = max(x_max, pos.0);
        y_min = min(y_min, pos.1);
        y_max = max(y_max, pos.1);
    }

    (x_min, y_min, x_max, y_max)
}

fn build_points(input: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    for line in input.lines() {
        let new = line
            .replace("position=<", " ")
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

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Point {
    fn update(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }

    fn rev(&mut self) {
        self.position.0 -= self.velocity.0;
        self.position.1 -= self.velocity.1;
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        let input = "position=< 9,  1> velocity=< 0,  2>
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
position=<-3,  6> velocity=< 2, -1>";

        assert_eq!(
            star_one(input),
            String::from(
                "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###\n"
            )
        )
    }

    #[test]
    fn test_star_two() {
        let input = "position=< 9,  1> velocity=< 0,  2>
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
position=<-3,  6> velocity=< 2, -1>";

        assert_eq!(star_two(input), 3)
    }
}
