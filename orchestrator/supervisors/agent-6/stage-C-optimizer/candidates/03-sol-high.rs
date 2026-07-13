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

pub fn optimize(program: &[Op]) -> Vec<Op> {
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

    let mut result = Vec::with_capacity(len);
    let mut old_to_new = vec![0; len + 1];
    let mut i = 0;

    while i < len {
        let new_index = result.len();
        old_to_new[i] = new_index;

        if i + 2 < len && !targeted[i + 1] && !targeted[i + 2] {
            if let (Op::Push(a), Op::Push(b)) = (&program[i], &program[i + 1]) {
                let folded = match &program[i + 2] {
                    Op::Add => Some(a.wrapping_add(*b)),
                    Op::Sub => Some(a.wrapping_sub(*b)),
                    Op::Mul => Some(a.wrapping_mul(*b)),
                    Op::Div if *b != 0 => Some(a.wrapping_div(*b)),
                    Op::Mod if *b != 0 => Some(a.wrapping_rem(*b)),
                    _ => None,
                };

                if let Some(value) = folded {
                    old_to_new[i + 1] = new_index;
                    old_to_new[i + 2] = new_index;
                    result.push(Op::Push(value));
                    i += 3;
                    continue;
                }
            }
        }

        if i + 1 < len && !targeted[i + 1] {
            match (&program[i], &program[i + 1]) {
                (Op::Push(value), Op::Neg) => {
                    old_to_new[i + 1] = new_index;
                    result.push(Op::Push(value.wrapping_neg()));
                    i += 2;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    old_to_new[i + 1] = new_index;
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        result.push(copy_op(&program[i]));
        i += 1;
    }

    old_to_new[len] = result.len();

    for op in &mut result {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= len => {
                *target = old_to_new[*target];
            }
            _ => {}
        }
    }

    result
}
