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

fn clone_program(program: &[Op]) -> Vec<Op> {
    program.iter().map(clone_op).collect()
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return (clone_program(program), false);
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut mapping = vec![0; length];
    let mut changed = false;
    let mut index = 0;

    while index < length {
        let folded = if index + 2 < length
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            match (
                &program[index],
                &program[index + 1],
                &program[index + 2],
            ) {
                (Op::Push(a), Op::Push(b), Op::Add) => {
                    Some((Op::Push((*a).wrapping_add(*b)), 3))
                }
                (Op::Push(a), Op::Push(b), Op::Sub) => {
                    Some((Op::Push((*a).wrapping_sub(*b)), 3))
                }
                (Op::Push(a), Op::Push(b), Op::Mul) => {
                    Some((Op::Push((*a).wrapping_mul(*b)), 3))
                }
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some((Op::Push((*a).wrapping_div(*b)), 3))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some((Op::Push((*a).wrapping_rem(*b)), 3))
                }
                _ => None,
            }
        } else {
            None
        };

        if let Some((replacement, width)) = folded {
            let position = output.len();
            mapping[index] = position;
            for offset in 1..width {
                mapping[index + offset] = position;
            }
            output.push(replacement);
            index += width;
            changed = true;
            continue;
        }

        if index + 1 < length
            && !targeted[index]
            && !targeted[index + 1]
            && matches!(
                (&program[index], &program[index + 1]),
                (Op::Push(_), Op::Pop)
            )
        {
            let position = output.len();
            mapping[index] = position;
            mapping[index + 1] = position;
            index += 2;
            changed = true;
            continue;
        }

        let folded_negation = if index + 1 < length && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    Some(Op::Push((*value).wrapping_neg()))
                }
                _ => None,
            }
        } else {
            None
        };

        if let Some(replacement) = folded_negation {
            let position = output.len();
            mapping[index] = position;
            mapping[index + 1] = position;
            output.push(replacement);
            index += 2;
            changed = true;
            continue;
        }

        mapping[index] = output.len();
        output.push(clone_op(&program[index]));
        index += 1;
    }

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                *target = mapping[old_target];
            }
            _ => {}
        }
    }

    (output, changed)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = clone_program(program);

    loop {
        let (next, changed) = optimize_pass(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}
