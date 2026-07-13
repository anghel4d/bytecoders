pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = Vec::with_capacity(program.len());
    for op in program {
        current.push(__ano_optimize_copy_op(op));
    }

    loop {
        let (next, changed) = __ano_optimize_pass(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}

fn __ano_optimize_copy_op(op: &Op) -> Op {
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

fn __ano_optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let len = program.len();
    let mut targeted = vec![false; len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < len => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut optimized = Vec::with_capacity(len);
    let mut mapping = vec![0usize; len + 1];
    let mut changed = false;
    let mut index = 0;

    while index < len {
        if index + 1 < len && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) => {
                    let position = optimized.len();
                    mapping[index] = position;
                    mapping[index + 1] = position;
                    index += 2;
                    changed = true;
                    continue;
                }
                (Op::Push(value), Op::Neg) => {
                    let position = optimized.len();
                    mapping[index] = position;
                    mapping[index + 1] = position;
                    optimized.push(Op::Push((*value).wrapping_neg()));
                    index += 2;
                    changed = true;
                    continue;
                }
                _ => {}
            }
        }

        if index + 2 < len && !targeted[index + 1] && !targeted[index + 2] {
            if let (Op::Push(left), Op::Push(right)) =
                (&program[index], &program[index + 1])
            {
                let folded = match &program[index + 2] {
                    Op::Add => Some((*left).wrapping_add(*right)),
                    Op::Sub => Some((*left).wrapping_sub(*right)),
                    Op::Mul => Some((*left).wrapping_mul(*right)),
                    Op::Div if *right != 0 => Some((*left).wrapping_div(*right)),
                    Op::Mod if *right != 0 => Some((*left).wrapping_rem(*right)),
                    _ => None,
                };

                if let Some(value) = folded {
                    let position = optimized.len();
                    mapping[index] = position;
                    mapping[index + 1] = position;
                    mapping[index + 2] = position;
                    optimized.push(Op::Push(value));
                    index += 3;
                    changed = true;
                    continue;
                }
            }
        }

        mapping[index] = optimized.len();
        optimized.push(__ano_optimize_copy_op(&program[index]));
        index += 1;
    }

    mapping[len] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                if old_target <= len {
                    *target = mapping[old_target];
                }
            }
            _ => {}
        }
    }

    (optimized, changed)
}
