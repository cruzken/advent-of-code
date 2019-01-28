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
    let mut header_pos = 0;
    let mut sum = 0;

    println!("{:?}", slice);
    while header_pos < slice.len() {
        let (children, md_length) = (slice[header_pos], slice[header_pos + 1] as usize);

        // base case
        if children < 1 {
        } else {
            // recursive case

            // make a slice of root slice minus the header
            // pass through fn children(slice, child_num)
            // fn children returns the sum calculated inside and header position
            println!("child slice to parse: {:?}", &slice[(header_pos + 2)..]);
            let (child_sum, ending_pos) = child_parser(&slice[(header_pos + 2)..], children);
            sum += child_sum;
            header_pos += 2 + ending_pos;
        }
        let md_slice = &slice[(header_pos + 1)..(header_pos + md_length + 1)];
        println!("slice to sum {:?}", md_slice);
        sum += md_sum(md_slice);
        header_pos += 2 + md_length;
    }

    println!("sum is {}", &sum);
    sum
}

fn child_parser(slice: &[u32], child_num: u32) -> (u32, usize) {
    let mut header_pos = 0;
    let mut sum = 0;

    for _i in 0..child_num {
        let (children, md_length) = (slice[header_pos], slice[header_pos + 1] as usize);
        if children < 1 {
            let md_slice = &slice[(header_pos + 2)..(header_pos + md_length + 2)];
            println!("slice to sum {:?}", md_slice);
            sum += md_sum(md_slice);
            header_pos += 2 + md_length;
        } else {
            // recursive case

            // make a slice of root slice minus the header
            // pass through fn children(slice, child_num)
            // fn children returns the sum calculated inside and header position
            let (child_sum, ending_pos) = child_parser(&slice[(header_pos + 2)..], children);
            sum += child_sum;
            header_pos += 2 + ending_pos;
            let (md_index1, md_index2) = ((header_pos), (header_pos + md_length));
            println!("slice is indexes [{} to {}]", md_index1, md_index2);
            let md_slice = &slice[md_index1..md_index2];
            println!("slice to sum {:?}", md_slice);
            sum += md_sum(md_slice);
        }
    }

    (sum, header_pos)
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
