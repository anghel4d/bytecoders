fn optimize_clone_op(op: &Op) -> Op {
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

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < length => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut removed = vec![false; length];
    let mut replacements: Vec<Option<Op>> = (0..length).map(|_| None).collect();
    let mut index = 0;

    while index < length {
        if index + 2 < length {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(a), Op::Push(b), Op::Add) => Some((*a).wrapping_add(*b)),
                (Op::Push(a), Op::Push(b), Op::Sub) => Some((*a).wrapping_sub(*b)),
                (Op::Push(a), Op::Push(b), Op::Mul) => Some((*a).wrapping_mul(*b)),
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some((*a).wrapping_div(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some((*a).wrapping_rem(*b))
                }
                _ => None,
            };

            if let Some(value) = folded {
                if !targeted[index] && !targeted[index + 1] && !targeted[index + 2] {
                    replacements[index] = Some(Op::Push(value));
                    removed[index + 1] = true;
                    removed[index + 2] = true;
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < length {
            let folded = match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => Some((*value).wrapping_neg()),
                _ => None,
            };

            if let Some(value) = folded {
                if !targeted[index] && !targeted[index + 1] {
                    replacements[index] = Some(Op::Push(value));
                    removed[index + 1] = true;
                    index += 2;
                    continue;
                }
            }

            let push_pop = match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) => true,
                _ => false,
            };

            if push_pop && !targeted[index] && !targeted[index + 1] {
                removed[index] = true;
                removed[index + 1] = true;
                index += 2;
                continue;
            }
        }

        index += 1;
    }

    let mut optimized = Vec::with_capacity(length);
    let mut old_to_new = vec![0usize; length];

    for old_index in 0..length {
        if removed[old_index] {
            continue;
        }

        old_to_new[old_index] = optimized.len();

        if let Some(replacement) = replacements[old_index].as_ref() {
            optimized.push(optimize_clone_op(replacement));
        } else {
            optimized.push(optimize_clone_op(&program[old_index]));
        }
    }

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < length => {
                *target = old_to_new[*target];
            }
            _ => {}
        }
    }

    optimized
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = Vec::with_capacity(program.len());

    for op in program {
        current.push(optimize_clone_op(op));
    }

    loop {
        let next = optimize_pass(&current);

        if next.len() == current.len() {
            return current;
        }

        current = next;
    }
}
