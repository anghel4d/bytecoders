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

fn optimize_has_target(targets: &[bool], start: usize, end: usize) -> bool {
    targets[start..end].iter().any(|targeted| *targeted)
}

fn optimize_fold_binary(op: &Op, left: i64, right: i64) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div => {
            if right == 0 {
                None
            } else if left == i64::MIN && right == -1 {
                Some(i64::MIN)
            } else {
                Some(left / right)
            }
        }
        Op::Mod => {
            if right == 0 {
                None
            } else if left == i64::MIN && right == -1 {
                Some(0)
            } else {
                Some(left % right)
            }
        }
        _ => None,
    }
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < length {
                    targeted[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut optimized = Vec::with_capacity(length);
    let mut old_to_new = vec![usize::MAX; length];
    let mut changed = false;
    let mut index = 0;

    while index < length {
        if index + 1 < length
            && !optimize_has_target(&targeted, index, index + 2)
            && matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop))
        {
            changed = true;
            index += 2;
            continue;
        }

        if index + 1 < length && !optimize_has_target(&targeted, index, index + 2) {
            if let (Op::Push(value), Op::Neg) = (&program[index], &program[index + 1]) {
                let new_index = optimized.len();
                optimized.push(Op::Push(value.wrapping_neg()));
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                changed = true;
                index += 2;
                continue;
            }
        }

        if index + 2 < length && !optimize_has_target(&targeted, index, index + 3) {
            if let (Op::Push(left), Op::Push(right), op) =
                (&program[index], &program[index + 1], &program[index + 2])
            {
                if let Some(value) = optimize_fold_binary(op, *left, *right) {
                    let new_index = optimized.len();
                    optimized.push(Op::Push(value));
                    old_to_new[index] = new_index;
                    old_to_new[index + 1] = new_index;
                    old_to_new[index + 2] = new_index;
                    changed = true;
                    index += 3;
                    continue;
                }
            }
        }

        old_to_new[index] = optimized.len();
        optimized.push(optimize_clone_op(&program[index]));
        index += 1;
    }

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < length {
                    let mapped = old_to_new[*target];
                    if mapped != usize::MAX {
                        *target = mapped;
                    }
                }
            }
            _ => {}
        }
    }

    (optimized, changed)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current: Vec<Op> = program.iter().map(optimize_clone_op).collect();

    loop {
        let (next, changed) = optimize_pass(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}
