const OPTIMIZER_REMOVED: usize = usize::MAX;

fn optimizer_copy_op(op: &Op) -> Op {
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

fn optimizer_fold_binary(lhs: i64, rhs: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(lhs.wrapping_add(rhs)),
        Op::Sub => Some(lhs.wrapping_sub(rhs)),
        Op::Mul => Some(lhs.wrapping_mul(rhs)),
        Op::Div if rhs != 0 => Some(lhs.wrapping_div(rhs)),
        Op::Mod if rhs != 0 => Some(lhs.wrapping_rem(rhs)),
        _ => None,
    }
}

fn optimizer_pass(program: &[Op]) -> Option<Vec<Op>> {
    let len = program.len();
    let mut targeted = vec![false; len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= len {
                    return None;
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut old_to_new = vec![OPTIMIZER_REMOVED; len];
    let mut optimized = Vec::with_capacity(len);
    let mut changed = false;
    let mut index = 0;

    while index < len {
        if let Op::Push(lhs) = &program[index] {
            if index + 2 < len {
                if let (Op::Push(rhs), operation) =
                    (&program[index + 1], &program[index + 2])
                {
                    if !targeted[index + 1]
                        && !targeted[index + 2]
                        && let Some(value) =
                            optimizer_fold_binary(*lhs, *rhs, operation)
                    {
                        old_to_new[index] = optimized.len();
                        optimized.push(Op::Push(value));
                        index += 3;
                        changed = true;
                        continue;
                    }
                }
            }

            if index + 1 < len {
                match &program[index + 1] {
                    Op::Neg if !targeted[index + 1] => {
                        old_to_new[index] = optimized.len();
                        optimized.push(Op::Push(lhs.wrapping_neg()));
                        index += 2;
                        changed = true;
                        continue;
                    }
                    Op::Pop if !targeted[index] && !targeted[index + 1] => {
                        index += 2;
                        changed = true;
                        continue;
                    }
                    _ => {}
                }
            }
        }

        old_to_new[index] = optimized.len();
        optimized.push(optimizer_copy_op(&program[index]));
        index += 1;
    }

    if !changed {
        return None;
    }

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= len || old_to_new[*target] == OPTIMIZER_REMOVED {
                    return None;
                }
                *target = old_to_new[*target];
            }
            _ => {}
        }
    }

    Some(optimized)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = Vec::with_capacity(program.len());

    for op in program {
        current.push(optimizer_copy_op(op));
    }

    loop {
        match optimizer_pass(&current) {
            Some(next) => current = next,
            None => return current,
        }
    }
}
