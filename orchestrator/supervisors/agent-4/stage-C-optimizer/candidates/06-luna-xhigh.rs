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

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
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

    let mut optimized = Vec::with_capacity(length);
    let mut old_to_new = vec![0; length + 1];
    let mut changed = false;
    let mut index = 0;

    while index < length {
        if index + 2 < length
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
                (&Op::Push(a), &Op::Push(b), &Op::Add) => {
                    Some(Op::Push(a.wrapping_add(b)))
                }
                (&Op::Push(a), &Op::Push(b), &Op::Sub) => {
                    Some(Op::Push(a.wrapping_sub(b)))
                }
                (&Op::Push(a), &Op::Push(b), &Op::Mul) => {
                    Some(Op::Push(a.wrapping_mul(b)))
                }
                (&Op::Push(a), &Op::Push(b), &Op::Div) if b != 0 => {
                    Some(Op::Push(a.wrapping_div(b)))
                }
                (&Op::Push(a), &Op::Push(b), &Op::Mod) if b != 0 => {
                    Some(Op::Push(a.wrapping_rem(b)))
                }
                _ => None,
            };

            if let Some(op) = folded {
                let new_index = optimized.len();
                optimized.push(op);
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                old_to_new[index + 2] = new_index;
                index += 3;
                changed = true;
                continue;
            }
        }

        if index + 1 < length && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (&Op::Push(value), &Op::Neg) => {
                    let new_index = optimized.len();
                    optimized.push(Op::Push(value.wrapping_neg()));
                    old_to_new[index] = new_index;
                    old_to_new[index + 1] = new_index;
                    index += 2;
                    changed = true;
                    continue;
                }
                (&Op::Push(_), &Op::Pop) => {
                    let new_index = optimized.len();
                    old_to_new[index] = new_index;
                    old_to_new[index + 1] = new_index;
                    index += 2;
                    changed = true;
                    continue;
                }
                _ => {}
            }
        }

        old_to_new[index] = optimized.len();
        optimized.push(copy_op(&program[index]));
        index += 1;
    }

    old_to_new[length] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= length => {
                *target = old_to_new[*target];
            }
            _ => {}
        }
    }

    (optimized, changed)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = Vec::with_capacity(program.len());

    for op in program {
        current.push(copy_op(op));
    }

    loop {
        let (next, changed) = optimize_pass(&current);

        if !changed {
            return next;
        }

        current = next;
    }
}
