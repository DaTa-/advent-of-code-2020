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
    let mut visited = vec![false; code.len()];
    let mut accumulator = 0;
    let mut instruction_ptr = 0usize;
    loop {
        if visited[instruction_ptr] {
            break;
        }
        visited[instruction_ptr] = true;
        let (opcode, arg) = &code[instruction_ptr];
        match opcode.as_str() {
            "acc" => {
                accumulator += arg;
                instruction_ptr += 1;
            }
            "jmp" => instruction_ptr = (instruction_ptr as isize + arg) as usize,
            "nop" => instruction_ptr += 1,
            _ => unreachable!(),
        }
    }
    println!("{}", accumulator);
}
