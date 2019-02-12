// Solved using summed-area table approach

#[allow(dead_code)]
pub fn star_one(serial_num: usize) -> (Option<(usize, usize)>, i32) {
    const LENGTH: usize = 300;
    let mut largest = std::i32::MIN;
    let mut largest_coord = None;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; LENGTH]; LENGTH];

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, el) in row.iter_mut().enumerate() {
            *el = power_level((x, y), serial_num);
        }
    }

    let summed_table = build_summed_area(&grid);

    for y in 0..=297 {
        for x in 0..=297 {
            let next = power_square(&summed_table, (x, y), 3);
            if next > largest {
                largest = next;
                largest_coord = Some((x, y));
            }
        }
    }

    (largest_coord, largest)
}

#[allow(dead_code)]
pub fn star_two(serial_num: usize) -> (Option<(usize, usize, usize)>, i32) {
    const LENGTH: usize = 300;
    let mut largest = std::i32::MIN;
    let mut largest_coord = None;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; LENGTH]; LENGTH];

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, el) in row.iter_mut().enumerate() {
            *el = power_level((x, y), serial_num);
        }
    }

    let summed_table = build_summed_area(&grid);

    for s in 1..=LENGTH {
        for y in 0..=(LENGTH - s) {
            for x in 0..=(LENGTH - s) {
                let next = power_square(&summed_table, (x, y), s);
                if next > largest {
                    largest = next;
                    largest_coord = Some((x, y, s));
                }
            }
        }
    }

    (largest_coord, largest)
}

fn power_square(grid: &[Vec<i32>], (x, y): (usize, usize), length: usize) -> i32 {
    let br = grid[y + length - 1][x + length - 1];

    // check if left exists
    let l = if x > 0 {
        grid[y + length - 1][x - 1]
    } else {
        0
    };

    // check if top exists
    let t = if y > 0 {
        grid[y - 1][x + length - 1]
    } else {
        0
    };

    // check if top-left exists
    let tl = if y > 0 && x > 0 {
        grid[y - 1][x - 1]
    } else {
        0
    };

    tl + br - t - l
}

fn build_summed_area(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut summed_table = grid.to_owned();

    for row in 0..summed_table.len() {
        for col in 0..summed_table[0].len() {
            if col != 0 {
                summed_table[row][col] += summed_table[row][col - 1];
            }
            if row != 0 {
                for i in 0..row {
                    summed_table[row][col] += grid[i][col];
                }
            }
        }
    }
    summed_table
}

fn power_level(coord: (usize, usize), serial_num: usize) -> i32 {
    let rack_id = coord.0 + 10;
    let calc = ((rack_id * coord.1) + serial_num) * rack_id;

    if calc < 100 {
        return 0;
    }

    let sting = calc.to_string();

    let hund = sting.chars().rev().nth(2).unwrap();
    hund.to_digit(10).unwrap() as i32 - 5
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(42), (Some((21, 61)), 30));
        assert_eq!(star_one(18), (Some((33, 45)), 29));
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(18), (Some((90, 269, 16)), 113));
        assert_eq!(star_two(42), (Some((232, 251, 12)), 119));
    }
}
