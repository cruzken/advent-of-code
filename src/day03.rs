use std::collections::{HashSet, HashMap};

#[allow(dead_code)]
pub fn star_one(input: &str) -> usize {
    let claims: HashSet<Claim> = parse_claims(&input);
    let mut grid: HashMap<(u32, u32), HashSet<u32>> = HashMap::new();
    for i in claims.iter() {
        insert_claim(&i, &mut grid);
    }
    
    grid.iter().filter(|(.., v)| v.len() > 1).count()
}

#[allow(dead_code)]
pub fn star_two(input: &str) -> u32 {
    let claims: HashSet<Claim> = parse_claims(&input);
    let mut grid: HashMap<(u32, u32), HashSet<u32>> = HashMap::new();
    for i in claims.iter() {
        insert_claim(&i, &mut grid);
    }
    let mut overlapped_ids: HashSet<u32> = HashSet::new();
    for (.., ids) in grid.iter().filter(|(.., v)| v.len() > 1) {
        for id in ids.iter() {
            overlapped_ids.insert(*id);
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
            let claim_ids = grid.entry(claim_coords).or_insert_with(HashSet::new);
            
            claim_ids.insert(claim.id);
        }
    }
}

fn parse_claims(input: &str) -> HashSet<Claim> {
    let mut the_list: HashSet<Claim> = HashSet::new();
    for claim_str in input.lines() {
        if claim_str.chars().count() > 0 {
            the_list.insert(claim_builder(claim_str));
        }
    }
    the_list
}

fn claim_builder(claim_str: &str) -> Claim {
    let claim_groups: Vec<u32> = claim_str
        .replace("#", "")
        .replace(" @ ", " ")
        .replace(",", " ")
        .replace(":", "")
        .replace("x", " ")
        .split_whitespace()
        .map(|x| parse_u32(x))
        .collect();
        
    Claim {
        id: claim_groups[0],
        left_edge: claim_groups[1],
        top_edge: claim_groups[2],
        width: claim_groups[3],
        height: claim_groups[4],
    }
}

fn parse_u32(slice: &str) -> u32 {
    slice.parse::<u32>().unwrap()
}

#[derive(PartialEq, Eq, Hash)]
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
