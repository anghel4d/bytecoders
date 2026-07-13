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

fn fold_binary(left: i64, right: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div if right != 0 => Some(left.wrapping_div(right)),
        Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
        _ => None,
    }
}

fn optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length + 1];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= length => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut mapping = vec![0; length + 1];
    let mut optimized = Vec::with_capacity(length);
    let mut changed = false;
    let mut index = 0;

    while index < length {
        if let Op::Push(value) = &program[index] {
            let value = *value;

            if index + 1 < length
                && matches!(&program[index + 1], Op::Pop)
                && !targeted[index]
                && !targeted[index + 1]
            {
                let output_index = optimized.len();
                mapping[index] = output_index;
                mapping[index + 1] = output_index;
                index += 2;
                changed = true;
                continue;
            }

            if index + 1 < length
                && matches!(&program[index + 1], Op::Neg)
                && !targeted[index + 1]
            {
                let output_index = optimized.len();
                mapping[index] = output_index;
                mapping[index + 1] = output_index;
                optimized.push(Op::Push(value.wrapping_neg()));
                index += 2;
                changed = true;
                continue;
            }

            if index + 2 < length {
                let right = match &program[index + 1] {
                    Op::Push(value) => Some(*value),
                    _ => None,
                };

                if let Some(right) = right {
                    if !targeted[index + 1]
                        && !targeted[index + 2]
                        && let Some(result) =
                            fold_binary(value, right, &program[index + 2])
                    {
                        let output_index = optimized.len();
                        mapping[index] = output_index;
                        mapping[index + 1] = output_index;
                        mapping[index + 2] = output_index;
                        optimized.push(Op::Push(result));
                        index += 3;
                        changed = true;
                        continue;
                    }
                }
            }
        }

        mapping[index] = optimized.len();
        optimized.push(copy_op(&program[index]));
        index += 1;
    }

    mapping[length] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= length => {
                let old_target = *target;
                *target = mapping[old_target];
            }
            _ => {}
        }
    }

    (optimized, changed)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = Vec::with_capacity(program.len());

    for op in program {
        current.push(copy_op(op));
    }

    loop {
        let (next, changed) = optimize_once(&current);

        if !changed {
            return current;
        }

        current = next;
    }
}
