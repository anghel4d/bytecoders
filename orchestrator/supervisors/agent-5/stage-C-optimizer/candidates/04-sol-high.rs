pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current: Vec<Op> = program.iter().map(copy_op).collect();

    loop {
        let (next, changed) = optimize_once(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}

fn optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
    let len = program.len();
    let mut targeted = vec![false; len + 1];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => Some(*target),
            _ => None,
        };

        if let Some(target) = target {
            if target <= len {
                targeted[target] = true;
            }
        }
    }

    let mut output = Vec::with_capacity(len);
    let mut old_to_new = vec![0; len + 1];
    let mut changed = false;
    let mut index = 0;

    while index < len {
        let new_index = output.len();

        if let Op::Push(a) = &program[index] {
            let a = *a;

            if index + 2 < len && !targeted[index + 1] && !targeted[index + 2] {
                if let Op::Push(b) = &program[index + 1] {
                    if let Some(value) = fold_binary(a, *b, &program[index + 2]) {
                        old_to_new[index] = new_index;
                        old_to_new[index + 1] = new_index;
                        old_to_new[index + 2] = new_index;
                        output.push(Op::Push(value));
                        index += 3;
                        changed = true;
                        continue;
                    }
                }
            }

            if index + 1 < len && !targeted[index + 1] {
                match &program[index + 1] {
                    Op::Neg => {
                        old_to_new[index] = new_index;
                        old_to_new[index + 1] = new_index;
                        output.push(Op::Push(a.wrapping_neg()));
                        index += 2;
                        changed = true;
                        continue;
                    }
                    Op::Pop => {
                        old_to_new[index] = new_index;
                        old_to_new[index + 1] = new_index;
                        index += 2;
                        changed = true;
                        continue;
                    }
                    _ => {}
                }
            }
        }

        old_to_new[index] = new_index;
        output.push(copy_op(&program[index]));
        index += 1;
    }

    old_to_new[len] = output.len();

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= len => {
                *target = old_to_new[*target];
            }
            _ => {}
        }
    }

    (output, changed)
}

fn fold_binary(a: i64, b: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(a.wrapping_add(b)),
        Op::Sub => Some(a.wrapping_sub(b)),
        Op::Mul => Some(a.wrapping_mul(b)),
        Op::Div if b != 0 => Some(a.wrapping_div(b)),
        Op::Mod if b != 0 => Some(a.wrapping_rem(b)),
        _ => None,
    }
}

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
