fn ano_peephole_clone(op: &Op) -> Op {
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

fn ano_peephole_fold_binary(left: i64, right: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div => {
            if right == 0 || (left == i64::MIN && right == -1) {
                None
            } else {
                Some(left / right)
            }
        }
        Op::Mod => {
            if right == 0 || (left == i64::MIN && right == -1) {
                None
            } else {
                Some(left % right)
            }
        }
        _ => None,
    }
}

fn ano_peephole_optimize_block(program: &[Op], start: usize, end: usize) -> Vec<Op> {
    let mut output = Vec::with_capacity(end - start);

    for op in &program[start..end] {
        output.push(ano_peephole_clone(op));

        loop {
            let len = output.len();

            let is_push_pop = if len >= 2 {
                matches!((&output[len - 2], &output[len - 1]), (Op::Push(_), Op::Pop))
            } else {
                false
            };

            if is_push_pop {
                output.truncate(len - 2);
                continue;
            }

            let negated = if len >= 2 {
                match (&output[len - 2], &output[len - 1]) {
                    (Op::Push(value), Op::Neg) => Some((*value).wrapping_neg()),
                    _ => None,
                }
            } else {
                None
            };

            if let Some(value) = negated {
                output.truncate(len - 2);
                output.push(Op::Push(value));
                continue;
            }

            let folded = if len >= 3 {
                match (&output[len - 3], &output[len - 2], &output[len - 1]) {
                    (Op::Push(left), Op::Push(right), op) => {
                        ano_peephole_fold_binary(*left, *right, op)
                    }
                    _ => None,
                }
            } else {
                None
            };

            if let Some(value) = folded {
                output.truncate(len - 3);
                output.push(Op::Push(value));
                continue;
            }

            break;
        }
    }

    output
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut boundaries = vec![false; program.len() + 1];
    boundaries[0] = true;
    boundaries[program.len()] = true;

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target > program.len() {
                    return program.iter().map(ano_peephole_clone).collect();
                }
                boundaries[*target] = true;
            }
            _ => {}
        }
    }

    let mut starts = Vec::new();
    for index in 0..=program.len() {
        if boundaries[index] {
            starts.push(index);
        }
    }

    let mut target_map = vec![0usize; program.len() + 1];
    let mut lowered = Vec::new();

    for pair in starts.windows(2) {
        let start = pair[0];
        let end = pair[1];

        target_map[start] = lowered.len();
        lowered.extend(ano_peephole_optimize_block(program, start, end));
    }

    if let Some(&start) = starts.last() {
        target_map[start] = lowered.len();
    }

    let mut result = Vec::with_capacity(lowered.len());

    for op in &lowered {
        match op {
            Op::Jmp(target) => result.push(Op::Jmp(target_map[*target])),
            Op::Jz(target) => result.push(Op::Jz(target_map[*target])),
            _ => result.push(ano_peephole_clone(op)),
        }
    }

    result
}
