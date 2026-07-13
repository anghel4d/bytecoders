pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = optimize_copy_program(program);

    loop {
        let (next, changed) = optimize_pass(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
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

    let mut mapping = vec![None; len];
    let mut output = Vec::with_capacity(len);
    let mut changed = false;
    let mut index = 0;

    while index < len {
        if index + 2 < len {
            let folded = match (
                &program[index],
                &program[index + 1],
                &program[index + 2],
            ) {
                (Op::Push(a), Op::Push(b), op) => {
                    optimize_fold_binary(op, *a, *b)
                }
                _ => None,
            };

            if let Some(value) = folded {
                if !targeted[index + 1] && !targeted[index + 2] {
                    mapping[index] = Some(output.len());
                    output.push(Op::Push(value));
                    index += 3;
                    changed = true;
                    continue;
                }
            }
        }

        if index + 1 < len {
            let folded = match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => Some((*value).wrapping_neg()),
                _ => None,
            };

            if let Some(value) = folded {
                if !targeted[index + 1] {
                    mapping[index] = Some(output.len());
                    output.push(Op::Push(value));
                    index += 2;
                    changed = true;
                    continue;
                }
            }

            if matches!(
                (&program[index], &program[index + 1]),
                (Op::Push(_), Op::Pop)
            ) && !targeted[index]
                && !targeted[index + 1]
            {
                index += 2;
                changed = true;
                continue;
            }
        }

        mapping[index] = Some(output.len());
        output.push(optimize_copy_op(&program[index]));
        index += 1;
    }

    if !changed {
        return (output, false);
    }

    let mut rewritten = Vec::with_capacity(output.len());

    for op in output {
        match op {
            Op::Jmp(target) => {
                if target < len {
                    match mapping[target] {
                        Some(mapped) => rewritten.push(Op::Jmp(mapped)),
                        None => return (optimize_copy_program(program), false),
                    }
                } else {
                    rewritten.push(Op::Jmp(target));
                }
            }
            Op::Jz(target) => {
                if target < len {
                    match mapping[target] {
                        Some(mapped) => rewritten.push(Op::Jz(mapped)),
                        None => return (optimize_copy_program(program), false),
                    }
                } else {
                    rewritten.push(Op::Jz(target));
                }
            }
            op => rewritten.push(op),
        }
    }

    (rewritten, true)
}

fn optimize_fold_binary(op: &Op, a: i64, b: i64) -> Option<i64> {
    match op {
        Op::Add => Some(a.wrapping_add(b)),
        Op::Sub => Some(a.wrapping_sub(b)),
        Op::Mul => Some(a.wrapping_mul(b)),
        Op::Div if b != 0 => Some(a.wrapping_div(b)),
        Op::Mod if b != 0 => Some(a.wrapping_rem(b)),
        _ => None,
    }
}

fn optimize_copy_program(program: &[Op]) -> Vec<Op> {
    program.iter().map(optimize_copy_op).collect()
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
