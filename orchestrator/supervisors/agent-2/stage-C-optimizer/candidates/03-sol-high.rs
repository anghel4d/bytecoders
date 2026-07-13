pub fn optimize(program: &[Op]) -> Vec<Op> {
    if program.iter().any(|op| match op {
        Op::Jmp(target) | Op::Jz(target) => *target > program.len(),
        _ => false,
    }) {
        return copy_program(program);
    }

    let mut current = copy_program(program);

    loop {
        let (next, changed) = optimize_pass(&current);
        current = next;

        if !changed {
            return current;
        }
    }
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let len = program.len();
    let mut targeted = vec![false; len + 1];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => targeted[*target] = true,
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(len);
    let mut old_to_new = vec![0; len + 1];
    let mut changed = false;
    let mut index = 0;

    while index < len {
        let output_index = output.len();

        if index + 2 < len && !targeted[index + 1] && !targeted[index + 2] {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(a), Op::Push(b), Op::Add) => Some(a.wrapping_add(*b)),
                (Op::Push(a), Op::Push(b), Op::Sub) => Some(a.wrapping_sub(*b)),
                (Op::Push(a), Op::Push(b), Op::Mul) => Some(a.wrapping_mul(*b)),
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some(a.wrapping_div(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some(a.wrapping_rem(*b))
                }
                _ => None,
            };

            if let Some(value) = folded {
                old_to_new[index] = output_index;
                old_to_new[index + 1] = output_index;
                old_to_new[index + 2] = output_index;
                output.push(Op::Push(value));
                index += 3;
                changed = true;
                continue;
            }
        }

        if index + 1 < len && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    old_to_new[index] = output_index;
                    old_to_new[index + 1] = output_index;
                    output.push(Op::Push(value.wrapping_neg()));
                    index += 2;
                    changed = true;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    old_to_new[index] = output_index;
                    old_to_new[index + 1] = output_index;
                    index += 2;
                    changed = true;
                    continue;
                }
                _ => {}
            }
        }

        old_to_new[index] = output_index;
        output.push(copy_op(&program[index]));
        index += 1;
    }

    old_to_new[len] = output.len();

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => *target = old_to_new[*target],
            _ => {}
        }
    }

    (output, changed)
}

fn copy_program(program: &[Op]) -> Vec<Op> {
    program.iter().map(copy_op).collect()
}

fn copy_op(op: &Op) -> Op {
    match op {
        Op::Push(value) => Op::Push(*value),
        Op::Pop => Op::Pop,
        Op::Add => Op::Add,
        Op::Sub => Op::Sub,
        Op::Mul => Op::Mul,
        Op::Div => Op::Div,
        Op::Mod => Op::Mod,
        Op::Neg => Op::Neg,
        Op::Dup => Op::Dup,
        Op::Swap => Op::Swap,
        Op::Jmp(target) => Op::Jmp(*target),
        Op::Jz(target) => Op::Jz(*target),
        Op::Print => Op::Print,
        Op::Halt => Op::Halt,
    }
}
