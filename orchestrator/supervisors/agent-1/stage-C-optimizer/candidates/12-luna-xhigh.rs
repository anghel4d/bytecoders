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

fn optimize_copy_program(program: &[Op]) -> Vec<Op> {
    program.iter().map(optimize_copy_op).collect()
}

fn optimize_fold_binary(a: i64, b: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(a.wrapping_add(b)),
        Op::Sub => Some(a.wrapping_sub(b)),
        Op::Mul => Some(a.wrapping_mul(b)),
        Op::Div => {
            if b == 0 {
                None
            } else if a == i64::MIN && b == -1 {
                Some(i64::MIN)
            } else {
                Some(a / b)
            }
        }
        Op::Mod => {
            if b == 0 {
                None
            } else if a == i64::MIN && b == -1 {
                Some(0)
            } else {
                Some(a % b)
            }
        }
        _ => None,
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return optimize_copy_program(program);
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
        if index + 1 < length {
            let removable = match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) => !targeted[index] && !targeted[index + 1],
                _ => false,
            };

            if removable {
                index += 2;
                continue;
            }
        }

        if index + 2 < length
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            let folded = match (
                &program[index],
                &program[index + 1],
                &program[index + 2],
            ) {
                (Op::Push(a), Op::Push(b), op) => {
                    optimize_fold_binary(*a, *b, op)
                }
                _ => None,
            };

            if let Some(value) = folded {
                let new_index = optimized.len();
                optimized.push(Op::Push(value));
                old_to_new[index] = Some(new_index);
                index += 3;
                continue;
            }
        }

        if index + 1 < length && !targeted[index + 1] {
            let folded = match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => Some(value.wrapping_neg()),
                _ => None,
            };

            if let Some(value) = folded {
                let new_index = optimized.len();
                optimized.push(Op::Push(value));
                old_to_new[index] = Some(new_index);
                index += 2;
                continue;
            }
        }

        let new_index = optimized.len();
        old_to_new[index] = Some(new_index);
        optimized.push(optimize_copy_op(&program[index]));
        index += 1;
    }

    for op in &optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if old_to_new[*target].is_none() {
                    return optimize_copy_program(program);
                }
            }
            _ => {}
        }
    }

    for op in &mut optimized {
        match op {
            Op::Jmp(target) => {
                *target = old_to_new[*target].unwrap();
            }
            Op::Jz(target) => {
                *target = old_to_new[*target].unwrap();
            }
            _ => {}
        }
    }

    optimized
}
