pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current: Vec<Op> = program.iter().map(optimize_copy_op).collect();

    loop {
        let next = optimize_once(&current);
        if next.len() == current.len() {
            return next;
        }
        current = next;
    }
}

fn optimize_once(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = vec![false; length];

    for instruction in program {
        match instruction {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return program.iter().map(optimize_copy_op).collect();
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut optimized = Vec::with_capacity(length);
    let mut old_to_new = vec![None; length];
    let mut index = 0;

    while index < length {
        let replacement = if index + 1 < length
            && !targeted[index]
            && !targeted[index + 1]
        {
            match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) => Some((2, None)),
                (Op::Push(value), Op::Neg) => {
                    Some((2, Some(Op::Push((*value).wrapping_neg()))))
                }
                _ if index + 2 < length && !targeted[index + 2] => {
                    match (
                        &program[index],
                        &program[index + 1],
                        &program[index + 2],
                    ) {
                        (Op::Push(left), Op::Push(right), operation) => {
                            optimize_fold_binary(*left, *right, operation)
                                .map(|result| (3, Some(result)))
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        } else {
            None
        };

        if let Some((width, replacement)) = replacement {
            if let Some(instruction) = replacement {
                optimized.push(instruction);
            }
            index += width;
        } else {
            old_to_new[index] = Some(optimized.len());
            optimized.push(optimize_copy_op(&program[index]));
            index += 1;
        }
    }

    for instruction in &mut optimized {
        match instruction {
            Op::Jmp(target) | Op::Jz(target) => {
                if let Some(Some(new_target)) = old_to_new.get(*target).copied() {
                    *target = new_target;
                } else {
                    return program.iter().map(optimize_copy_op).collect();
                }
            }
            _ => {}
        }
    }

    optimized
}

fn optimize_fold_binary(left: i64, right: i64, operation: &Op) -> Option<Op> {
    match operation {
        Op::Add => Some(Op::Push(left.wrapping_add(right))),
        Op::Sub => Some(Op::Push(left.wrapping_sub(right))),
        Op::Mul => Some(Op::Push(left.wrapping_mul(right))),
        Op::Div if right != 0 => Some(Op::Push(left.wrapping_div(right))),
        Op::Mod if right != 0 => Some(Op::Push(left.wrapping_rem(right))),
        _ => None,
    }
}

fn optimize_copy_op(operation: &Op) -> Op {
    match operation {
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
