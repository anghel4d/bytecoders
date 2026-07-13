fn ano_opt_clone(op: &Op) -> Op {
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

fn ano_opt_fold_binary(left: i64, right: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div if right != 0 => Some(left.wrapping_div(right)),
        Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
        _ => None,
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target > length {
                    return program.iter().map(ano_opt_clone).collect();
                }
                if *target < length {
                    targeted[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut target_map = vec![0usize; length + 1];
    let mut index = 0;

    while index < length {
        if index + 2 < length
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            if let (Op::Push(left), Op::Push(right), operation) =
                (&program[index], &program[index + 1], &program[index + 2])
            {
                if let Some(value) = ano_opt_fold_binary(*left, *right, operation) {
                    let position = output.len();
                    target_map[index] = position;
                    target_map[index + 1] = position;
                    target_map[index + 2] = position;
                    output.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < length && !targeted[index + 1] {
            if let (Op::Push(value), Op::Neg) =
                (&program[index], &program[index + 1])
            {
                let position = output.len();
                target_map[index] = position;
                target_map[index + 1] = position;
                output.push(Op::Push(value.wrapping_neg()));
                index += 2;
                continue;
            }

            if let (Op::Push(_), Op::Pop) =
                (&program[index], &program[index + 1])
            {
                let position = output.len();
                target_map[index] = position;
                target_map[index + 1] = position;
                index += 2;
                continue;
            }
        }

        target_map[index] = output.len();
        output.push(ano_opt_clone(&program[index]));
        index += 1;
    }

    target_map[length] = output.len();

    for op in &mut output {
        match op {
            Op::Jmp(target) => {
                let old_target = *target;
                *target = target_map[old_target];
            }
            Op::Jz(target) => {
                let old_target = *target;
                *target = target_map[old_target];
            }
            _ => {}
        }
    }

    output
}
