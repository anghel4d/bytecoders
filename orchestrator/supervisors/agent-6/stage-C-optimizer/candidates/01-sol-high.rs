pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = clone_program(program);

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
    let mut targeted = vec![false; len];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => *target,
            _ => continue,
        };

        if target >= len {
            return (clone_program(program), false);
        }
        targeted[target] = true;
    }

    let mut old_to_new = vec![0; len + 1];
    let mut result = Vec::with_capacity(len);
    let mut changed = false;
    let mut i = 0;

    while i < len {
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
                    let new_index = result.len();
                    old_to_new[i] = new_index;
                    old_to_new[i + 1] = new_index;
                    old_to_new[i + 2] = new_index;
                    result.push(Op::Push(value));
                    i += 3;
                    changed = true;
                    continue;
                }
            }
        }

        if i + 1 < len && !targeted[i + 1] {
            if let (Op::Push(value), Op::Neg) = (&program[i], &program[i + 1]) {
                let new_index = result.len();
                old_to_new[i] = new_index;
                old_to_new[i + 1] = new_index;
                result.push(Op::Push(value.wrapping_neg()));
                i += 2;
                changed = true;
                continue;
            }
        }

        if i + 1 < len && !targeted[i] && !targeted[i + 1] {
            if matches!((&program[i], &program[i + 1]), (Op::Push(_), Op::Pop)) {
                let new_index = result.len();
                old_to_new[i] = new_index;
                old_to_new[i + 1] = new_index;
                i += 2;
                changed = true;
                continue;
            }
        }

        old_to_new[i] = result.len();
        result.push(clone_op(&program[i]));
        i += 1;
    }

    old_to_new[len] = result.len();

    for op in &mut result {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = old_to_new[*target];
            }
            _ => {}
        }
    }

    (result, changed)
}

fn clone_program(program: &[Op]) -> Vec<Op> {
    program.iter().map(clone_op).collect()
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
