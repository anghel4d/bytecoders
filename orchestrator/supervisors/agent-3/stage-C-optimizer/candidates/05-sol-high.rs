fn clone_optimizer_op(op: &Op) -> Op {
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
    let mut is_jump_target = vec![false; len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < len => {
                is_jump_target[*target] = true;
            }
            _ => {}
        }
    }

    let mut old_to_new = vec![0usize; len + 1];
    let mut optimized = Vec::with_capacity(len);
    let mut index = 0;

    while index < len {
        old_to_new[index] = optimized.len();

        if index + 2 < len
            && !is_jump_target[index + 1]
            && !is_jump_target[index + 2]
        {
            if let (Op::Push(a), Op::Push(b), op) = (
                &program[index],
                &program[index + 1],
                &program[index + 2],
            ) {
                let folded = match op {
                    Op::Add => Some((*a).wrapping_add(*b)),
                    Op::Sub => Some((*a).wrapping_sub(*b)),
                    Op::Mul => Some((*a).wrapping_mul(*b)),
                    Op::Div if *b != 0 => Some((*a).wrapping_div(*b)),
                    Op::Mod if *b != 0 => Some((*a).wrapping_rem(*b)),
                    _ => None,
                };

                if let Some(value) = folded {
                    optimized.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < len && !is_jump_target[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    optimized.push(Op::Push((*value).wrapping_neg()));
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

        optimized.push(clone_optimizer_op(&program[index]));
        index += 1;
    }

    old_to_new[len] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                if old_target <= len {
                    *target = old_to_new[old_target];
                }
            }
            _ => {}
        }
    }

    optimized
}
