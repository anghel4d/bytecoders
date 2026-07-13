fn optimize_clone_op(op: &Op) -> Op {
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

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return program.iter().map(optimize_clone_op).collect();
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut positions = vec![usize::MAX; length];
    let mut index = 0;

    while index < length {
        if length - index >= 3
            && !targeted[index]
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            let replacement = match (
                &program[index],
                &program[index + 1],
                &program[index + 2],
            ) {
                (Op::Push(a), Op::Push(b), Op::Add) => {
                    Some(Op::Push((*a).wrapping_add(*b)))
                }
                (Op::Push(a), Op::Push(b), Op::Sub) => {
                    Some(Op::Push((*a).wrapping_sub(*b)))
                }
                (Op::Push(a), Op::Push(b), Op::Mul) => {
                    Some(Op::Push((*a).wrapping_mul(*b)))
                }
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some(Op::Push((*a).wrapping_div(*b)))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some(Op::Push((*a).wrapping_rem(*b)))
                }
                _ => None,
            };

            if let Some(op) = replacement {
                positions[index] = output.len();
                output.push(op);
                index += 3;
                continue;
            }
        }

        if length - index >= 2 && !targeted[index] && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    positions[index] = output.len();
                    output.push(Op::Push((*value).wrapping_neg()));
                    index += 2;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    index += 2;
                    continue;
                }
                _ => {}
            }
        }

        positions[index] = output.len();
        output.push(optimize_clone_op(&program[index]));
        index += 1;
    }

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length || positions[*target] == usize::MAX {
                    return program.iter().map(optimize_clone_op).collect();
                }
                *target = positions[*target];
            }
            _ => {}
        }
    }

    output
}
