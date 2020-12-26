fn main() {
    let code = String::from_utf8(std::fs::read("input/day8").unwrap())
        .unwrap()
        .split_terminator("\n")
        .map(|instruction| {
            let mut instruction = instruction.split_whitespace();
            (
                instruction.next().unwrap().to_owned(),
                instruction.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(_, isize)>>();
    let answer = code
        .iter()
        .enumerate()
        .find_map(|(i, (opcode, _))| {
            if opcode == "acc" {
                return None;
            }

            let mut visited = vec![false; code.len()];
            let mut accumulator = 0;
            let mut instruction_ptr = 0usize;
            let success_replacement = loop {
                if instruction_ptr == code.len() {
                    break true;
                }
                if visited[instruction_ptr] {
                    break false;
                }
                visited[instruction_ptr] = true;
                let (opcode, arg) = &code[instruction_ptr];

                let opcode = if instruction_ptr == i {
                    match opcode.as_str() {
                        "jmp" => "nop",
                        "nop" => "jmp",
                        _ => unreachable!(),
                    }
                } else {
                    opcode.as_str()
                };

                match opcode {
                    "acc" => {
                        accumulator += arg;
                        instruction_ptr += 1;
                    }
                    "jmp" => instruction_ptr = (instruction_ptr as isize + arg) as usize,
                    "nop" => instruction_ptr += 1,
                    _ => unreachable!(),
                }
            };
            Some(accumulator).filter(|_| success_replacement)
        })
        .unwrap();
    println!("{}", answer);
}
