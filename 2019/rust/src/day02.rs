enum Operation {
    Add,
    Multi,
    Terminate,
}

fn get_command(opcode: usize) -> Operation {
    use self::Operation::{Add, Multi, Terminate};
    match opcode {
        1 => Add,
        2 => Multi,
        99 => Terminate,
        _ => unreachable!(),
    }
}

fn final_state(input: &Vec<usize>) -> usize {
    use self::Operation::{Add, Multi, Terminate};
    let mut opcodes = input.clone();
    let mut ptr = 0;

    loop {
        match get_command(opcodes[ptr]) {
            Terminate => break,
            x => {
                let (first_addr, second_addr, output_pos) =
                    (opcodes[ptr + 1], opcodes[ptr + 2], opcodes[ptr + 3]);

                let output_val = match x {
                    Add => opcodes[first_addr] + opcodes[second_addr],
                    Multi => opcodes[first_addr] * opcodes[second_addr],
                    Terminate => unreachable!(),
                };
                opcodes[output_pos] = output_val;
            }
        }
        ptr += 4;
    }
    opcodes[0]
}
// Part 1
pub fn star_one(input: &str) -> usize {
    let opcodes: Vec<_> = input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    final_state(&opcodes)
}
// Part 2
pub fn star_two(input: &str) -> usize {
    let opcodes: Vec<_> = input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = opcodes.clone();
            mem[1] = noun;
            mem[2] = verb;

            if final_state(&mem) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);

        //1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
        assert_eq!(star_one("1,0,0,0,99"), 2);
        //2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
        assert_eq!(star_one("2,3,0,3,99"), 2);
        //2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
        assert_eq!(star_one("2,4,4,5,99,0"), 2);
        //1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
        assert_eq!(star_one("1,1,1,4,99,5,6,0,99"), 30);
    }

    //#[test]
    //fn test_star_two() {
    //    assert_eq!(star_two("1,0,0,0,99"), 2)
    //}
}
