pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current: Vec<Op> = program.iter().map(clone_op).collect();

    loop {
        let old_len = current.len();
        let next = optimize_once(&current);

        if next.len() == old_len {
            return next;
        }

        current = next;
    }
}

fn optimize_once(program: &[Op]) -> Vec<Op> {
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
    let mut old_to_new = vec![0; len + 1];
    let mut index = 0;

    while index < len {
        if index + 2 < len && !targeted[index + 1] && !targeted[index + 2] {
            if let (Op::Push(a), Op::Push(b)) = (&program[index], &program[index + 1]) {
                if let Some(value) = fold_binary(*a, *b, &program[index + 2]) {
                    let new_index = optimized.len();
                    old_to_new[index] = new_index;
                    old_to_new[index + 1] = new_index;
                    old_to_new[index + 2] = new_index;
                    optimized.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < len && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    let new_index = optimized.len();
                    old_to_new[index] = new_index;
                    old_to_new[index + 1] = new_index;
                    optimized.push(Op::Push(value.wrapping_neg()));
                    index += 2;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    let new_index = optimized.len();
                    old_to_new[index] = new_index;
                    old_to_new[index + 1] = new_index;
                    index += 2;
                    continue;
                }
                _ => {}
            }
        }

        old_to_new[index] = optimized.len();
        optimized.push(clone_op(&program[index]));
        index += 1;
    }

    old_to_new[len] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                if old_target <= len {
                    *target = old_to_new[old_target];
                }
            }
            _ => {}
        }
    }

    optimized
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

fn clone_op(op: &Op) -> Op {
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
