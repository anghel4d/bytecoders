pub fn optimize(program: &[Op]) -> Vec<Op> {
    let (mut current, mut changed) = optimize_pass(program);

    while changed {
        let result = optimize_pass(&current);
        current = result.0;
        changed = result.1;
    }

    current
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let len = program.len();
    let mut targeted = vec![false; len + 1];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => *target,
            _ => continue,
        };

        if target > len {
            return (program.iter().map(copy_op).collect(), false);
        }

        targeted[target] = true;
    }

    let mut old_to_new = vec![0; len + 1];
    let mut output = Vec::with_capacity(len);
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
                    let new_index = output.len();
                    old_to_new[i] = new_index;
                    old_to_new[i + 1] = new_index;
                    old_to_new[i + 2] = new_index;
                    output.push(Op::Push(value));
                    i += 3;
                    changed = true;
                    continue;
                }
            }
        }

        if i + 1 < len && !targeted[i + 1] {
            match (&program[i], &program[i + 1]) {
                (Op::Push(value), Op::Neg) => {
                    let new_index = output.len();
                    old_to_new[i] = new_index;
                    old_to_new[i + 1] = new_index;
                    output.push(Op::Push(value.wrapping_neg()));
                    i += 2;
                    changed = true;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    let new_index = output.len();
                    old_to_new[i] = new_index;
                    old_to_new[i + 1] = new_index;
                    i += 2;
                    changed = true;
                    continue;
                }
                _ => {}
            }
        }

        old_to_new[i] = output.len();
        output.push(copy_op(&program[i]));
        i += 1;
    }

    old_to_new[len] = output.len();

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = old_to_new[*target];
            }
            _ => {}
        }
    }

    (output, changed)
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
