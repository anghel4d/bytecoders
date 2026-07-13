pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current: Vec<Op> = program.iter().map(optimize_copy_op).collect();

    loop {
        let (next, changed) = optimize_pass(&current);
        if !changed {
            return current;
        }
        current = next;
    }
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return (program.iter().map(optimize_copy_op).collect(), false);
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut old_to_new = vec![usize::MAX; length];
    let mut changed = false;
    let mut index = 0;

    while index < length {
        let mut replacement: Option<(Op, usize)> = None;

        if length - index >= 3 {
            replacement = match (&program[index], &program[index + 1], &program[index + 2]) {
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
            };
        }

        if replacement.is_none() && length - index >= 2 {
            replacement = match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => Some((Op::Push((*value).wrapping_neg()), 2)),
                _ => None,
            };
        }

        if let Some((replacement, width)) = replacement {
            if !targeted[index + 1..index + width]
                .iter()
                .any(|targeted| *targeted)
            {
                let destination = output.len();
                output.push(replacement);

                for old_index in index..index + width {
                    old_to_new[old_index] = destination;
                }

                index += width;
                changed = true;
                continue;
            }
        }

        if length - index >= 2
            && matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop))
            && !targeted[index..index + 2]
                .iter()
                .any(|targeted| *targeted)
        {
            let destination = output.len();
            old_to_new[index] = destination;
            old_to_new[index + 1] = destination;
            index += 2;
            changed = true;
            continue;
        }

        old_to_new[index] = output.len();
        output.push(optimize_copy_op(&program[index]));
        index += 1;
    }

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                *target = old_to_new[old_target];
            }
            _ => {}
        }
    }

    (output, changed)
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
