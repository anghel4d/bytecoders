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

fn optimize_fold_binary(first: &Op, second: &Op, operation: &Op) -> Option<i64> {
    match (first, second, operation) {
        (&Op::Push(a), &Op::Push(b), &Op::Add) => Some(a.wrapping_add(b)),
        (&Op::Push(a), &Op::Push(b), &Op::Sub) => Some(a.wrapping_sub(b)),
        (&Op::Push(a), &Op::Push(b), &Op::Mul) => Some(a.wrapping_mul(b)),
        (&Op::Push(a), &Op::Push(b), &Op::Div) if b != 0 => Some(a.wrapping_div(b)),
        (&Op::Push(a), &Op::Push(b), &Op::Mod) if b != 0 => Some(a.wrapping_rem(b)),
        _ => None,
    }
}

fn optimize_fold_unary(value: &Op, operation: &Op) -> Option<i64> {
    match (value, operation) {
        (&Op::Push(value), &Op::Neg) => Some(value.wrapping_neg()),
        _ => None,
    }
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut jump_targets = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < length {
                    jump_targets[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut old_to_new = vec![0usize; length + 1];
    let mut changed = false;
    let mut index = 0;

    while index < length {
        if index + 2 < length
            && !jump_targets[index + 1]
            && !jump_targets[index + 2]
        {
            if let Some(value) = optimize_fold_binary(
                &program[index],
                &program[index + 1],
                &program[index + 2],
            ) {
                let new_index = output.len();
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                old_to_new[index + 2] = new_index;
                output.push(Op::Push(value));
                index += 3;
                changed = true;
                continue;
            }
        }

        if index + 1 < length && !jump_targets[index + 1] {
            if let Some(value) =
                optimize_fold_unary(&program[index], &program[index + 1])
            {
                let new_index = output.len();
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                output.push(Op::Push(value));
                index += 2;
                changed = true;
                continue;
            }
        }

        if index + 1 < length
            && !jump_targets[index]
            && !jump_targets[index + 1]
        {
            let is_push_pop = match (&program[index], &program[index + 1]) {
                (&Op::Push(_), &Op::Pop) => true,
                _ => false,
            };

            if is_push_pop {
                let new_index = output.len();
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                index += 2;
                changed = true;
                continue;
            }
        }

        old_to_new[index] = output.len();
        output.push(optimize_clone_op(&program[index]));
        index += 1;
    }

    old_to_new[length] = output.len();

    if changed {
        for op in &mut output {
            match op {
                Op::Jmp(target) | Op::Jz(target) => {
                    if *target <= length {
                        *target = old_to_new[*target];
                    }
                }
                _ => {}
            }
        }
    }

    (output, changed)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = {
        let mut output = Vec::with_capacity(program.len());
        for op in program {
            output.push(optimize_clone_op(op));
        }
        output
    };

    loop {
        let (next, changed) = optimize_pass(&current);
        if !changed {
            return current;
        }
        current = next;
    }
}
