pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = optimize_pass(program);

    while current.len() < program.len() {
        let next = optimize_pass(&current);
        if next.len() == current.len() {
            break;
        }
        current = next;
    }

    current
}

fn optimize_pass(program: &[Op]) -> Vec<Op> {
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

    let mut old_to_new = vec![0; len + 1];
    let mut result = Vec::with_capacity(len);
    let mut index = 0;

    while index < len {
        let new_index = result.len();
        old_to_new[index] = new_index;

        if index + 2 < len && !targeted[index + 1] && !targeted[index + 2] {
            if let (Op::Push(a), Op::Push(b)) = (&program[index], &program[index + 1]) {
                let folded = match &program[index + 2] {
                    Op::Add => Some(a.wrapping_add(*b)),
                    Op::Sub => Some(a.wrapping_sub(*b)),
                    Op::Mul => Some(a.wrapping_mul(*b)),
                    Op::Div if *b != 0 => Some(a.wrapping_div(*b)),
                    Op::Mod if *b != 0 => Some(a.wrapping_rem(*b)),
                    _ => None,
                };

                if let Some(value) = folded {
                    old_to_new[index + 1] = new_index;
                    old_to_new[index + 2] = new_index;
                    result.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < len && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    old_to_new[index + 1] = new_index;
                    result.push(Op::Push(value.wrapping_neg()));
                    index += 2;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    old_to_new[index + 1] = new_index;
                    index += 2;
                    continue;
                }
                _ => {}
            }
        }

        result.push(copy_op(&program[index]));
        index += 1;
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
