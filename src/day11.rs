
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: u32 ) -> (Option<(u32, u32)>, i32) {
    let mut largest = std::i32::MIN;
    let mut largest_coord = None;
    for y in 1..=297 {
        for x in 1..=297 {
            let next = power_square((x,y), input);
            if next > largest {
                largest = next;
                largest_coord = Some((x, y));
            }
        }
    }

   (largest_coord, largest) 
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: u32) -> i64 {
    0
}

fn power_square(coord: (u32, u32), serial_num: u32) -> i32 {
    let mut sum = 0;
    for x in coord.0..(coord.0 + 3) {
        for y in coord.1..(coord.1 + 3) { 
            sum += power_level((x, y), serial_num);
        }
    }
    sum
}

fn power_level(coord: (u32, u32), serial_num: u32) -> i32 {
    let rack_id = coord.0 + 10;

    let calc = ((rack_id * coord.1) + serial_num) * rack_id;

    let sting = calc.to_string();
    if sting.len() < 3 {
        return 0;
    }

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
        assert_eq!(star_two(11),  1)
    }
}
