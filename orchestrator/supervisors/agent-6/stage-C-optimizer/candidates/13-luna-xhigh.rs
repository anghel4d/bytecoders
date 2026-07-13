enum OptimizePlan {
    Keep,
    Replace(Op),
    Remove,
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

fn optimize_fold_binary(a: i64, b: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(a.wrapping_add(b)),
        Op::Sub => Some(a.wrapping_sub(b)),
        Op::Mul => Some(a.wrapping_mul(b)),
        Op::Div if b != 0 => Some(a.wrapping_div(b)),
        Op::Mod if b != 0 => Some(a.wrapping_rem(b)),
        _ => None,
    }
}

fn optimize_rewrite_op(op: &Op, targets: &[usize]) -> Op {
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
        Op::Jmp(target) => Op::Jmp(targets[*target]),
        Op::Jz(target) => Op::Jz(targets[*target]),
        Op::Print => Op::Print,
        Op::Halt => Op::Halt,
    }
}

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => Some(*target),
            _ => None,
        };

        if let Some(target) = target {
            if target >= length {
                return program.iter().map(optimize_copy_op).collect();
            }
            targeted[target] = true;
        }
    }

    let mut plan = Vec::with_capacity(length);
    for _ in 0..length {
        plan.push(OptimizePlan::Keep);
    }

    let mut index = 0;
    while index < length {
        if index + 2 < length
            && !targeted[index]
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(a), Op::Push(b), op) => {
                    optimize_fold_binary(*a, *b, op)
                }
                _ => None,
            };

            if let Some(value) = folded {
                plan[index] = OptimizePlan::Replace(Op::Push(value));
                plan[index + 1] = OptimizePlan::Remove;
                plan[index + 2] = OptimizePlan::Remove;
                index += 3;
                continue;
            }
        }

        if index + 1 < length && !targeted[index] && !targeted[index + 1] {
            let is_push_pop = match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) => true,
                _ => false,
            };

            if is_push_pop {
                plan[index] = OptimizePlan::Remove;
                plan[index + 1] = OptimizePlan::Remove;
                index += 2;
                continue;
            }
        }

        if let Op::Push(value) = &program[index] {
            if index + 1 < length
                && !targeted[index]
                && !targeted[index + 1]
                && matches!(&program[index + 1], Op::Neg)
            {
                plan[index] = OptimizePlan::Replace(Op::Push(value.wrapping_neg()));
                plan[index + 1] = OptimizePlan::Remove;
                index += 2;
                continue;
            }
        }

        index += 1;
    }

    let mut targets = vec![0; length];
    let mut output_length = 0;

    for index in 0..length {
        targets[index] = output_length;

        let removed = match &plan[index] {
            OptimizePlan::Remove => true,
            _ => false,
        };

        if !removed {
            output_length += 1;
        }
    }

    let mut optimized = Vec::with_capacity(output_length);

    for index in 0..length {
        match &plan[index] {
            OptimizePlan::Keep => {
                optimized.push(optimize_rewrite_op(&program[index], &targets));
            }
            OptimizePlan::Replace(op) => {
                optimized.push(optimize_rewrite_op(op, &targets));
            }
            OptimizePlan::Remove => {}
        }
    }

    optimized
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = program.iter().map(optimize_copy_op).collect::<Vec<_>>();

    loop {
        let next = optimize_pass(&current);

        if next.len() >= current.len() {
            return current;
        }

        current = next;
    }
}
