use std::collections::HashSet;

#[allow(dead_code)]
pub fn star_one(input: &str) -> usize {
    let claims: Vec<Claim> = parse_claims(&input);
    let mut overlapped_claims: HashSet<(u32, u32)> = HashSet::new();
    for claim1 in 0..claims.len() {
        for claim2 in (claim1 + 1)..claims.len() {
            for overlap in get_overlap(&claims[claim1], &claims[claim2]) {
                overlapped_claims.insert(overlap);
            }
        }
    }
    overlapped_claims.len()
}

fn parse_claims(input: &str) -> Vec<Claim> {
    let mut the_list: Vec<Claim> = Vec::new();
    for claim_str in input.lines() {
        if claim_str.chars().count() > 0 {
            the_list.push(claim_builder(claim_str));
        }
    }
    the_list
}

fn claim_builder(claim_str: &str) -> Claim {
    let parsed_claim = claim_str
        .replace("#", "")
        .replace(" @ ", " ")
        .replace(",", " ")
        .replace(":", "")
        .replace("x", " ");

    let claim_groups: Vec<u32> = parsed_claim
        .split_whitespace()
        .map(|x| parse_u32(x))
        .collect();

    let (left_edge, top_edge, width, height) = (
        claim_groups[1],
        claim_groups[2],
        claim_groups[3],
        claim_groups[4],
    );

    Claim {
        left_edge,
        top_edge,
        width,
        height,
    }
}

fn get_overlap(claim1: &Claim, claim2: &Claim) -> HashSet<(u32, u32)> {
    let overlap: HashSet<(u32, u32)> = HashSet::new();
    if has_overlap(claim1, claim2) {
        return get_overlap_coords(claim1, claim2);
    }
    overlap
}

fn has_overlap(claim1: &Claim, claim2: &Claim) -> bool {
    let claim1_top_left = (claim1.left_edge, claim1.top_edge);
    let claim1_bot_right = (
        claim1.left_edge + claim1.width - 1,
        claim1.top_edge + claim1.height - 1,
    );
    let claim2_top_left = (claim2.left_edge, claim2.top_edge);
    let claim2_bot_right = (
        claim2.left_edge + claim2.width - 1,
        claim2.top_edge + claim2.height - 1,
    );

    if claim1_top_left.0 > claim2_bot_right.0 || claim2_top_left.0 > claim1_bot_right.0 {
        return false;
    }
    if claim1_top_left.1 > claim2_bot_right.1 || claim2_top_left.1 > claim1_bot_right.1 {
        return false;
    }
    true
}

// lazy and inefficient way
fn get_overlap_coords(claim1: &Claim, claim2: &Claim) -> HashSet<(u32, u32)> {
    let mut overlap_coords: HashSet<(u32, u32)> = HashSet::new();
    let mut claim1_squares: HashSet<(u32, u32)> = HashSet::new();
    let mut claim2_squares: HashSet<(u32, u32)> = HashSet::new();

    for y in claim1.top_edge..(claim1.top_edge + claim1.height) {
        for x in claim1.left_edge..(claim1.left_edge + claim1.width) {
            claim1_squares.insert((x, y));
        }
    }

    for y in claim2.top_edge..(claim2.top_edge + claim2.height) {
        for x in claim2.left_edge..(claim2.left_edge + claim2.width) {
            claim2_squares.insert((x, y));
        }
    }
    for coords in claim1_squares.intersection(&claim2_squares) {
        overlap_coords.insert(*coords);
    }
    overlap_coords
}

fn parse_u32(slice: &str) -> u32 {
    slice.parse::<u32>().unwrap()
}

struct Claim {
    left_edge: u32,
    top_edge: u32,
    width: u32,
    height: u32,
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn star_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 4);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(""), 1)
    }
}
