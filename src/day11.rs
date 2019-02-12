#[allow(dead_code)]
pub fn star_one(input: u32 ) -> (Option<(u32, u32)>, i32) {
    let mut largest = std::i32::MIN;
    let mut largest_coord = None;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 300]; 300];

    for y in 0..300 {
        for x in 0..300 {
            grid[x as usize][y as usize] = power_level((x, y), input);
        }
    }
    for y in 1..=297 {
        for x in 1..=297 {
            let next = power_square((x + 1,y + 1,3), &grid);
            if next > largest {
                largest = next;
                largest_coord = Some((x, y));
            }
        }
    }

   (largest_coord, largest) 
}

// Needs optimization, couldnt get it to finish on my machine
#[allow(dead_code)]
pub fn star_two(input: u32) -> (Option<(u32, u32, u32)>, i32) {
    let mut largest = std::i32::MIN;
    let mut largest_coord = None;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 300]; 300];

    for y in 0..300 {
        for x in 0..300 {
            grid[x as usize][y as usize] = power_level((x, y), input);
        }
    }

    let summed_table = build_summed_table(&grid);
   (largest_coord, largest) 
}

fn build_summed_table(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let mut summed_table = grid.clone();

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

fn power_square(coord: (u32, u32, u32), grid: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for x in coord.0..(coord.0 + coord.2) {
        for y in coord.1..(coord.1 + coord.2) { 
            sum += grid[x as usize - 1][y as usize - 1];
        }
    }
    sum
}

fn power_level(coord: (u32, u32), serial_num: u32) -> i32 {
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
        assert_eq!(star_two(18), (Some((90,269,16)), 113));
    }
}
