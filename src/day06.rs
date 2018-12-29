use std::collections::{BTreeMap, HashMap, HashSet};

#[allow(dead_code)]
pub fn star_one(input: &str) -> u32 {
    // parse input to coords
    let coords = build_coords(input);

    // get grid's boundaries
    let bounds = grid_bounds(&coords);

    // build grid data calculating distance per point per grid coord
    let grid = build_grid(&coords, &bounds);

    // find the points with finite areas
    let inf = infinite_points(&grid, &bounds);
    let finite = coords.difference(&inf).collect::<HashSet<_>>();

    // get areas of coords with finite areas
    *finite_areas(&grid, &finite).iter().max().unwrap()
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str, limit: i32) -> usize {
    // parse input to coords
    let coords = build_coords(input);

    // get grid's boundaries
    let bounds = grid_bounds(&coords);

    // build grid data calculating distance per point per grid coord
    let grid = build_grid(&coords, &bounds);

    // return region area that is under the limit
    grid.iter()
        .map(|(.., dlist)| {
            dlist
                .iter()
                .fold(0, |acc, (dist, points)| acc + (dist * points.len() as i32))
        })
        .filter(|x| *x < limit)
        .count()
}

fn infinite_points(
    grid: &HashMap<Point, BTreeMap<i32, HashSet<Point>>>,
    bounds: &(i32, i32, i32, i32),
) -> HashSet<Point> {
    let mut infinite_points = HashSet::new();

    for x in bounds.0..=bounds.1 {
        for y in bounds.2..=bounds.3 {
            // Only check the parameter
            if x == bounds.0 || x == bounds.1 || y == bounds.2 || y == bounds.3 {
                let point = Point { x, y };
                let (.., dlist) = grid.get(&point).unwrap().iter().next().unwrap();

                if dlist.len() == 1 {
                    let copy = dlist.iter().next().unwrap();
                    infinite_points.insert(Point {
                        x: copy.x,
                        y: copy.y,
                    });
                }
            }
        }
    }
    infinite_points
}

fn finite_areas(
    grid: &HashMap<Point, BTreeMap<i32, HashSet<Point>>>,
    finite_points: &HashSet<&Point>,
) -> HashSet<u32> {
    let mut distances = HashSet::new();

    for point in finite_points.iter() {
        let mut count = 0;

        for (.., dlist) in grid.iter() {
            let (.., closest) = dlist.iter().next().unwrap();
            if closest.len() == 1 && closest.contains(point) {
                count += 1;
            }
        }

        distances.insert(count);
    }
    distances
}

fn build_grid(
    coords: &HashSet<Point>,
    bounds: &(i32, i32, i32, i32),
) -> HashMap<Point, BTreeMap<i32, HashSet<Point>>> {
    let mut grid = HashMap::new();

    for x in bounds.0..=bounds.1 {
        for y in bounds.2..=bounds.3 {
            let point = Point { x, y };
            let distances = grid.entry(Point { x, y }).or_insert_with(BTreeMap::new);

            for (distance, entry) in coords.iter().map(|p| (p.distance(&point), p)) {
                let distances_list = distances.entry(distance).or_insert_with(HashSet::new);
                distances_list.insert(Point {
                    x: entry.x,
                    y: entry.y,
                });
            }
        }
    }
    grid
}

// need to refactor
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

#[derive(Hash, PartialEq, Eq, Debug)]
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
        assert_eq!(
            star_one(
                "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
            ),
            17
        )
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(
                "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
                32
            ),
            16
        )
    }
}
