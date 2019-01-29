#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> u32 {
    let data: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    node_build(&data, 0).0
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

fn node_build(slice: &[u32], position: usize) -> (u32, usize) {
    // get header
    let (children, md_length) = (slice[position], slice[position + 1] as usize);

    let mut md_pos = position + 2;
    let mut sum = 0;

    for _ in 0..children {
        let (child_sum, next_position) = node_build(slice, md_pos);
        md_pos = next_position;
        sum += child_sum;
    }

    sum += md_sum(&slice[md_pos..(md_pos + md_length)]);

    (sum, md_pos + md_length)
}

fn md_sum(slice: &[u32]) -> u32 {
    let mut sum = 0;
    for el in slice {
        sum += el;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
