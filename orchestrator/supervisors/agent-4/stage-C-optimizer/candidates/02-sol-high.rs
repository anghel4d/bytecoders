fn clone_op(op: &Op) -> Op {
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
    let len = program.len();
    let mut is_target = vec![false; len + 1];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => *target,
            _ => continue,
        };

        if target > len {
            return program.iter().map(clone_op).collect();
        }

        is_target[target] = true;
    }

    let mut optimized = Vec::with_capacity(len);
    let mut new_index = vec![0usize; len + 1];
    let mut index = 0;

    while index < len {
        let output_index = optimized.len();
        new_index[index] = output_index;

        if index + 2 < len && !is_target[index + 1] && !is_target[index + 2] {
            if let (Op::Push(a), Op::Push(b)) = (&program[index], &program[index + 1]) {
                let folded = match &program[index + 2] {
                    Op::Add => Some((*a).wrapping_add(*b)),
                    Op::Sub => Some((*a).wrapping_sub(*b)),
                    Op::Mul => Some((*a).wrapping_mul(*b)),
                    Op::Div if *b != 0 => Some((*a).wrapping_div(*b)),
                    Op::Mod if *b != 0 => Some((*a).wrapping_rem(*b)),
                    _ => None,
                };

                if let Some(value) = folded {
                    new_index[index + 1] = output_index;
                    new_index[index + 2] = output_index;
                    optimized.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < len && !is_target[index + 1] {
            if let Op::Push(value) = &program[index] {
                match &program[index + 1] {
                    Op::Neg => {
                        new_index[index + 1] = output_index;
                        optimized.push(Op::Push((*value).wrapping_neg()));
                        index += 2;
                        continue;
                    }
                    Op::Pop => {
                        new_index[index + 1] = output_index;
                        index += 2;
                        continue;
                    }
                    _ => {}
                }
            }
        }

        optimized.push(clone_op(&program[index]));
        index += 1;
    }

    new_index[len] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = new_index[*target];
            }
            _ => {}
        }
    }

    optimized
}
