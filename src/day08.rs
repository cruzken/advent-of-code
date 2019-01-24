#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_one(input: &str) -> i64 {
    let data: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    node_build(&data);

    0
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

fn node_build(slice: &[u32]) -> u32 {
    // get header
    let children = slice[0];
    let md_entries = slice[1];

    // recursive fn:
    // get header info (child_num, md_num)
    // if has children, make a children slice, else get slice of metadata
    // get the children slice header info
    // if has children, make a child slice, else get slice of metadata
    // else slice of metadata is after header
    1
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 1)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
