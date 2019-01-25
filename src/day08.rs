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
    let mut sum = 0;
    let (children, md_entries) = (slice[0], slice[1] as usize);

    if children > 0 {
        children_parser(&slice[2..]); // returns md_sum, and position where children slice ends
    } else {
        let mut next_header = 0;
        loop {
            let md_slice = &slice[2..md_entries + 2];
            sum += md_sum(md_slice);
            next_header = 2 + md_entries;
            if next_header >= slice.len() {
                break;
            }
        }
        // get metadata slice and sum the total
        // get next header position
        // break loop if next header position is bigger than slice length
    }
    // recursive fn:
    // get header info (child_num, md_num)
    // if has children, make a children slice, else get slice of metadata
    // get the children slice header info
    // if has children, make a child slice, else get slice of metadata
    // else slice of metadata is after header
    1
}

fn children_parser(slice: &[u32]) -> (u32, usize) {
    let (children, md_entries) = (slice[0], slice[1]);
    if children > 0 {
        children_parser(&slice[2..]);
    }
    (0, 0)
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
        assert_eq!(star_one("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 1)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
