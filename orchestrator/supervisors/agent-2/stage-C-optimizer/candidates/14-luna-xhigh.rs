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

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut jump_target = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < length => {
                jump_target[*target] = true;
            }
            _ => {}
        }
    }

    let mut optimized = Vec::with_capacity(length);
    let mut old_to_new = vec![0; length + 1];
    let mut index = 0;

    while index < length {
        if index + 2 < length
            && !jump_target[index + 1]
            && !jump_target[index + 2]
        {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(a), Op::Push(b), Op::Add) => {
                    Some((*a).wrapping_add(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Sub) => {
                    Some((*a).wrapping_sub(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Mul) => {
                    Some((*a).wrapping_mul(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some((*a).wrapping_div(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some((*a).wrapping_rem(*b))
                }
                _ => None,
            };

            if let Some(value) = folded {
                let new_index = optimized.len();
                optimized.push(Op::Push(value));
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                old_to_new[index + 2] = new_index;
                index += 3;
                continue;
            }
        }

        if index + 1 < length
            && !jump_target[index]
            && !jump_target[index + 1]
        {
            if let (Op::Push(_), Op::Pop) = (&program[index], &program[index + 1]) {
                let new_index = optimized.len();
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                index += 2;
                continue;
            }
        }

        if index + 1 < length && !jump_target[index + 1] {
            if let (Op::Push(value), Op::Neg) = (&program[index], &program[index + 1]) {
                let new_index = optimized.len();
                optimized.push(Op::Push((*value).wrapping_neg()));
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                index += 2;
                continue;
            }
        }

        old_to_new[index] = optimized.len();
        optimized.push(copy_op(&program[index]));
        index += 1;
    }

    old_to_new[length] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= length => {
                let old_target = *target;
                *target = old_to_new[old_target];
            }
            _ => {}
        }
    }

    optimized
}
