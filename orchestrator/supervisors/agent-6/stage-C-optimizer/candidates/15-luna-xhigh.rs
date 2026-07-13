fn optimize_fold_binary(left: i64, right: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div => {
            if right == 0 {
                None
            } else {
                Some(left.wrapping_div(right))
            }
        }
        Op::Mod => {
            if right == 0 {
                None
            } else {
                Some(left.wrapping_rem(right))
            }
        }
        _ => None,
    }
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

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut jump_targets = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < length => {
                jump_targets[*target] = true;
            }
            _ => {}
        }
    }

    let mut optimized = Vec::with_capacity(length);
    let mut target_map = vec![0usize; length + 1];
    let mut index = 0;

    while index < length {
        if length - index >= 3
            && !jump_targets[index + 1]
            && !jump_targets[index + 2]
        {
            if let (Op::Push(left), Op::Push(right)) =
                (&program[index], &program[index + 1])
            {
                if let Some(value) =
                    optimize_fold_binary(*left, *right, &program[index + 2])
                {
                    let output_index = optimized.len();
                    target_map[index] = output_index;
                    target_map[index + 1] = output_index;
                    target_map[index + 2] = output_index;
                    optimized.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        if length - index >= 2 && !jump_targets[index + 1] {
            if let Op::Push(value) = &program[index] {
                match &program[index + 1] {
                    Op::Neg => {
                        let output_index = optimized.len();
                        target_map[index] = output_index;
                        target_map[index + 1] = output_index;
                        optimized.push(Op::Push(value.wrapping_neg()));
                        index += 2;
                        continue;
                    }
                    Op::Pop => {
                        let output_index = optimized.len();
                        target_map[index] = output_index;
                        target_map[index + 1] = output_index;
                        index += 2;
                        continue;
                    }
                    _ => {}
                }
            }
        }

        target_map[index] = optimized.len();
        optimized.push(optimize_copy_op(&program[index]));
        index += 1;
    }

    target_map[length] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= length => {
                *target = target_map[*target];
            }
            _ => {}
        }
    }

    optimized
}
