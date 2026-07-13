pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut optimized: Vec<Op> = program.iter().map(optimize_copy_op).collect();

    loop {
        let next = optimize_once(&optimized);
        if next.len() >= optimized.len() {
            return optimized;
        }
        optimized = next;
    }
}

fn optimize_once(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut jump_targets = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < length => {
                jump_targets[*target] = true;
            }
            _ => {}
        }
    }

    let mut old_to_new = vec![0; length];
    let mut output = Vec::with_capacity(length);
    let mut jumps = Vec::new();
    let mut index = 0;

    while index < length {
        if index + 2 < length && !jump_targets[index + 1] && !jump_targets[index + 2] {
            if let (Op::Push(left), Op::Push(right)) =
                (&program[index], &program[index + 1])
            {
                if let Some(value) =
                    optimize_fold_binary(&program[index + 2], *left, *right)
                {
                    let position = output.len();
                    old_to_new[index] = position;
                    old_to_new[index + 1] = position;
                    old_to_new[index + 2] = position;
                    output.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < length && !jump_targets[index + 1] {
            if let Op::Push(value) = &program[index] {
                if let Op::Neg = &program[index + 1] {
                    let position = output.len();
                    old_to_new[index] = position;
                    old_to_new[index + 1] = position;
                    output.push(Op::Push(value.wrapping_neg()));
                    index += 2;
                    continue;
                }
            }
        }

        if index + 1 < length && !jump_targets[index + 1] {
            if let Op::Push(_) = &program[index] {
                if let Op::Pop = &program[index + 1] {
                    let position = output.len();
                    old_to_new[index] = position;
                    old_to_new[index + 1] = position;
                    index += 2;
                    continue;
                }
            }
        }

        let position = output.len();
        old_to_new[index] = position;

        match &program[index] {
            Op::Jmp(target) | Op::Jz(target) => {
                jumps.push((position, *target));
                output.push(optimize_copy_op(&program[index]));
            }
            _ => output.push(optimize_copy_op(&program[index])),
        }

        index += 1;
    }

    for (position, target) in jumps {
        if target < length {
            let rewritten_target = old_to_new[target];
            match &mut output[position] {
                Op::Jmp(target) | Op::Jz(target) => *target = rewritten_target,
                _ => {}
            }
        }
    }

    output
}

fn optimize_fold_binary(op: &Op, left: i64, right: i64) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div if right != 0 => Some(left.wrapping_div(right)),
        Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
        _ => None,
    }
}

fn optimize_copy_op(op: &Op) -> Op {
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
