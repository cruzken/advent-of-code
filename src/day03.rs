use std::collections::HashSet;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn star_one(input: &str) -> usize {
    let claims: Vec<Claim> = parse_claims(&input);
    let mut overlapped_claims: HashSet<(u32, u32)> = HashSet::new();
    for i in 0..claims.len() {
        for j in (i + 1)..claims.len() {
            for overlap in get_overlap(&claims[i], &claims[j]) {
                overlapped_claims.insert(overlap);
            }
        }
    }
    overlapped_claims.len()
}

#[allow(dead_code)]
pub fn star_two(input: &str) -> u32 {
    let claims: Vec<Claim> = parse_claims(&input);
    let mut grid: HashMap<(u32, u32), HashSet<u32>> = HashMap::new();
    for i in 0..claims.len() {
        insert_claim(&claims[i], &mut grid);
    }
    let mut overlapped_ids: HashSet<u32> = HashSet::new();
    for (.., ids) in grid.iter() {
        if ids.len() > 1 {
            for id in ids.iter() {
                overlapped_ids.insert(*id);
            }
        }
    }
    let claim_ids: HashSet<u32> = claims.into_iter().map(|x| x.id).collect();
    let diff: Vec<_> = claim_ids.difference(&overlapped_ids).collect();
    if diff.len() == 1 {
        return *diff[0];
    }
    0
}

fn insert_claim(claim: &Claim, grid: &mut HashMap<(u32, u32), HashSet<u32>>) {
    for y in claim.top_edge..(claim.top_edge + claim.height) {
        for x in claim.left_edge..(claim.left_edge + claim.width) {
            let claim_coords = (x, y);
            
            let claim_ids = grid.entry(claim_coords).or_insert(HashSet::new());
            claim_ids.insert(claim.id);
            
        }
    }
        
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

    let (id, left_edge, top_edge, width, height) = (
        claim_groups[0],
        claim_groups[1],
        claim_groups[2],
        claim_groups[3],
        claim_groups[4],
    );

    Claim {
        id,
        left_edge,
        top_edge,
        width,
        height,
    }
}

fn get_overlap(i: &Claim, j: &Claim) -> HashSet<(u32, u32)> {
    let overlap: HashSet<(u32, u32)> = HashSet::new();
    if has_overlap(i, j) {
        return get_overlap_coords(i, j);
    }
    overlap
}

fn has_overlap(i: &Claim, j: &Claim) -> bool {
    let i_top_left = (i.left_edge, i.top_edge);
    let i_bot_right = (i.left_edge + i.width - 1, i.top_edge + i.height - 1);
    let j_top_left = (j.left_edge, j.top_edge);
    let j_bot_right = (j.left_edge + j.width - 1, j.top_edge + j.height - 1);

    if i_top_left.0 > j_bot_right.0 || j_top_left.0 > i_bot_right.0 {
        return false;
    }
    if i_top_left.1 > j_bot_right.1 || j_top_left.1 > i_bot_right.1 {
        return false;
    }
    true
}

// lazy and inefficient way
fn get_overlap_coords(i: &Claim, j: &Claim) -> HashSet<(u32, u32)> {
    let mut overlap_coords: HashSet<(u32, u32)> = HashSet::new();
    let mut i_squares: HashSet<(u32, u32)> = HashSet::new();
    let mut j_squares: HashSet<(u32, u32)> = HashSet::new();

    for y in i.top_edge..(i.top_edge + i.height) {
        for x in i.left_edge..(i.left_edge + i.width) {
            i_squares.insert((x, y));
        }
    }

    for y in j.top_edge..(j.top_edge + j.height) {
        for x in j.left_edge..(j.left_edge + j.width) {
            j_squares.insert((x, y));
        }
    }
    for coords in i_squares.intersection(&j_squares) {
        overlap_coords.insert(*coords);
    }
    overlap_coords
}

fn parse_u32(slice: &str) -> u32 {
    slice.parse::<u32>().unwrap()
}

struct Claim {
    id: u32,
    left_edge: u32,
    top_edge: u32,
    width: u32,
    height: u32,
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
        assert_eq!(star_two("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 3);
    }
}
