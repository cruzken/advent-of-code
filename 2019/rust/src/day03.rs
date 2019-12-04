use self::Direction::{Down, Left, Right, Up};
use std::collections::{HashMap, HashSet};

enum Direction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

fn get_direction(dirCode: &str) -> Direction {
    let mut dir = dirCode.chars();
    let letter = dir.next().unwrap();
    let number = dir.collect::<String>().parse::<u32>().unwrap();
    match letter {
        'U' => Up(number),
        'D' => Down(number),
        'L' => Left(number),
        'R' => Right(number),
        _ => unreachable!(),
    }
}

fn wire_points(line: &str) -> HashMap<i32, HashSet<i32>> {
    let mut points: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut coord = (0, 0);

    for dir in line.split(',') {
        build_points(get_direction(&dir), &mut coord, &mut points);
    }
    points
}

fn build_points(d: Direction, c: &mut (i32, i32), p: &mut HashMap<i32, HashSet<i32>>) {
    match d {
        Up(x) => {
            for y in c.1..=(c.1 + x as i32) {
                c.1 = y;
                let set = p.entry(c.0).or_insert(HashSet::new());
                set.insert(c.1);
            }
        }
        Down(x) => {
            for y in ((c.1 - x as i32)..=(c.1)).rev() {
                c.1 = y;
                let set = p.entry(c.0).or_insert(HashSet::new());
                set.insert(c.1);
            }
        }
        Left(x) => {
            for x in ((c.0 - x as i32)..=(c.0)).rev() {
                c.0 = x;
                let set = p.entry(c.0).or_insert(HashSet::new());
                set.insert(c.1);
            }
        }
        Right(x) => {
            for x in c.0..=(c.0 + x as i32) {
                c.0 = x;
                let set = p.entry(c.0).or_insert(HashSet::new());
                set.insert(c.1);
            }
        }
    }
}

fn found_intersect(c: &(i32, i32), p: &HashMap<i32, HashSet<i32>>) -> bool {
    if let Some(x) = p.get(&c.0) {
        if let Some(_) = x.get(&c.1) {
            return true;
        }
    }
    false
}

fn get_intersection(line: &str, p: &HashMap<i32, HashSet<i32>>) -> Vec<(i32, i32)> {
    let mut c = (0, 0);
    let mut points: Vec<(i32, i32)> = vec![];

    for dir in line.split(',') {
        match get_direction(&dir) {
            Up(x) => {
                for y in (c.1 + 1)..=(c.1 + x as i32) {
                    c.1 = y;
                    if found_intersect(&c, &p) {
                        points.push((c.0, c.1));
                    }
                }
            }
            Down(x) => {
                for y in ((c.1 - x as i32)..=(c.1 - 1)).rev() {
                    c.1 = y;
                    if found_intersect(&c, &p) {
                        points.push((c.0, c.1));
                    }
                }
            }
            Left(x) => {
                for x in ((c.0 - x as i32)..=(c.0 - 1)).rev() {
                    c.0 = x;
                    if found_intersect(&c, &p) {
                        points.push((c.0, c.1));
                    }
                }
            }
            Right(x) => {
                for x in (c.0 + 1)..=(c.0 + x as i32) {
                    c.0 = x;
                    if found_intersect(&c, &p) {
                        points.push((c.0, c.1));
                    }
                }
            }
        }
    }
    points
}

fn steps_per_intersect(w1: &str, w2: &str, intersections: &Vec<(i32, i32)>) -> u32 {
    let mut first_steps: Vec<u32> = vec![];
    for point in intersections {
        first_steps.push(get_steps(&w1, &point));
    }

    let mut second_steps: Vec<u32> = vec![];
    for point in intersections {
        second_steps.push(get_steps(&w2, &point));
    }
    first_steps.iter().zip(second_steps).map(|(x, y)| x + y).min().unwrap()
}

fn get_steps(wire: &str, p: &(i32, i32)) -> u32 {
    let mut steps = vec![];
    let mut c: (i32, i32) = (0, 0);
    for dir in wire.split(',') {
        match get_direction(&dir) {
            Up(x) => {
                if c.0 == p.0 && c.1 + (x as i32) >= p.1 {
                    steps.push((p.1 - c.1).abs() as u32);
                    break;
                } else {
                    c.1 += x as i32;
                    steps.push(x);
                }
            }
            Down(x) => {
                if c.0 == p.0 && (c.1 - (x as i32) <= p.1 && p.1 < c.1) {
                    steps.push((p.1 - c.1).abs() as u32);
                    break;
                } else {
                    c.1 -= x as i32;
                    steps.push(x);
                }
            }
            Left(x) => {
                if c.1 == p.1 && c.0 - (x as i32) <= p.0 {
                    steps.push((p.0 - c.0).abs() as u32);
                    break;
                } else {
                    steps.push(x);
                    c.0 -= x as i32;
                }
            }
            Right(x) => {
                if c.1 == p.1 && c.0 + (x as i32) >= p.0 {
                    steps.push((p.0 - c.0).abs() as u32);
                    break;
                } else {
                    steps.push(x);
                    c.0 += x as i32;
                }
            }
        }
    }
    steps.iter().sum()
}

pub fn star_one(input: &str) -> i32 {
    let mut lines = input.lines();
    let wire1 = lines.next().unwrap();
    let wire2 = lines.next().unwrap();
    let points1 = wire_points(wire1);
    let intersections = get_intersection(wire2, &points1);
    let closest = intersections.iter().fold(i32::max_value(), |closest, x| {
        closest.min(x.0.abs() + x.1.abs())
    });
    closest
}

pub fn star_two(input: &str) -> u32 {
    let mut lines = input.lines();
    let wire1 = lines.next().unwrap();
    let wire2 = lines.next().unwrap();
    let points1 = wire_points(wire1);
    let intersections = get_intersection(wire2, &points1);
    let steps = steps_per_intersect(wire1, wire2, &intersections);
    steps
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("R8,U5,L5,D3\nU7,R6,D4,L4"), 6)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
        assert_eq!(star_two("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), 610);
    }
}
