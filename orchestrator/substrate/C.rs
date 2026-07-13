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

fn reduce_tail(code: &mut Vec<Op>) {
    loop {
        let len = code.len();

        if len >= 2 {
            if matches!((&code[len - 2], &code[len - 1]), (Op::Push(_), Op::Pop)) {
                code.truncate(len - 2);
                continue;
            }

            if let (Op::Push(value), Op::Neg) = (&code[len - 2], &code[len - 1]) {
                let value = value.wrapping_neg();
                code.truncate(len - 2);
                code.push(Op::Push(value));
                continue;
            }
        }

        if len >= 3 {
            let folded = match (&code[len - 3], &code[len - 2], &code[len - 1]) {
                (Op::Push(a), Op::Push(b), Op::Add) => Some(a.wrapping_add(*b)),
                (Op::Push(a), Op::Push(b), Op::Sub) => Some(a.wrapping_sub(*b)),
                (Op::Push(a), Op::Push(b), Op::Mul) => Some(a.wrapping_mul(*b)),
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some(a.wrapping_div(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some(a.wrapping_rem(*b))
                }
                _ => None,
            };

            if let Some(value) = folded {
                code.truncate(len - 3);
                code.push(Op::Push(value));
                continue;
            }
        }

        break;
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let program_len = program.len();
    let mut boundaries = vec![false; program_len + 1];
    boundaries[0] = true;
    boundaries[program_len] = true;

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => *target,
            _ => continue,
        };

        if target <= program_len {
            boundaries[target] = true;
        }
    }

    let mut old_to_new = vec![usize::MAX; program_len + 1];
    let mut optimized = Vec::with_capacity(program_len);
    let mut start = 0;

    while start < program_len {
        old_to_new[start] = optimized.len();

        let mut end = start + 1;
        while !boundaries[end] {
            end += 1;
        }

        let mut segment = Vec::with_capacity(end - start);
        for op in &program[start..end] {
            segment.push(copy_op(op));
            reduce_tail(&mut segment);
        }
        optimized.extend(segment);

        start = end;
    }

    old_to_new[program_len] = optimized.len();

    for op in &mut optimized {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => target,
            _ => continue,
        };

        let old_target = *target;
        if old_target <= program_len {
            *target = old_to_new[old_target];
        }
    }

    optimized
}
