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

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => Some(*target),
            _ => None,
        };

        if let Some(target) = target {
            if target < length {
                targeted[target] = true;
            }
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut targets = vec![0usize; length];
    let mut changed = false;
    let mut index = 0;

    while index < length {
        if index + 1 < length && !targeted[index + 1] {
            if let (Op::Push(_), Op::Pop) = (&program[index], &program[index + 1]) {
                let output_index = output.len();
                targets[index] = output_index;
                targets[index + 1] = output_index;
                index += 2;
                changed = true;
                continue;
            }

            if let (Op::Push(value), Op::Neg) = (&program[index], &program[index + 1]) {
                let output_index = output.len();
                targets[index] = output_index;
                targets[index + 1] = output_index;
                output.push(Op::Push(value.wrapping_neg()));
                index += 2;
                changed = true;
                continue;
            }
        }

        if index + 2 < length && !targeted[index + 1] && !targeted[index + 2] {
            if let (Op::Push(a), Op::Push(b), op) =
                (&program[index], &program[index + 1], &program[index + 2])
            {
                if let Some(value) = optimize_fold_binary(*a, *b, op) {
                    let output_index = output.len();
                    targets[index] = output_index;
                    targets[index + 1] = output_index;
                    targets[index + 2] = output_index;
                    output.push(Op::Push(value));
                    index += 3;
                    changed = true;
                    continue;
                }
            }
        }

        targets[index] = output.len();
        output.push(optimize_clone_op(&program[index]));
        index += 1;
    }

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < length {
                    *target = targets[*target];
                }
            }
            _ => {}
        }
    }

    (output, changed)
}
