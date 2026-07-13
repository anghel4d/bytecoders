pub fn optimize(program: &[Op]) -> Vec<Op> {
    let (mut result, mut changed) = optimize_pass(program);

    while changed {
        let (next, next_changed) = optimize_pass(&result);
        result = next;
        changed = next_changed;
    }

    result
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
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

    let mut output = Vec::with_capacity(len);
    let mut target_map = vec![0; len + 1];
    let mut changed = false;
    let mut index = 0;

    while index < len {
        let folded_binary = if len - index >= 3
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            match (
                &program[index],
                &program[index + 1],
                &program[index + 2],
            ) {
                (Op::Push(left), Op::Push(right), op) => {
                    fold_binary(*left, *right, op)
                }
                _ => None,
            }
        } else {
            None
        };

        if let Some(value) = folded_binary {
            let output_index = output.len();
            target_map[index] = output_index;
            target_map[index + 1] = output_index;
            target_map[index + 2] = output_index;
            output.push(Op::Push(value));
            index += 3;
            changed = true;
            continue;
        }

        let folded_neg = if len - index >= 2 && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => Some((*value).wrapping_neg()),
                _ => None,
            }
        } else {
            None
        };

        if let Some(value) = folded_neg {
            let output_index = output.len();
            target_map[index] = output_index;
            target_map[index + 1] = output_index;
            output.push(Op::Push(value));
            index += 2;
            changed = true;
            continue;
        }

        let removes_push_pop = if len - index >= 2 && !targeted[index + 1] {
            matches!(
                (&program[index], &program[index + 1]),
                (Op::Push(_), Op::Pop)
            )
        } else {
            false
        };

        if removes_push_pop {
            let output_index = output.len();
            target_map[index] = output_index;
            target_map[index + 1] = output_index;
            index += 2;
            changed = true;
            continue;
        }

        target_map[index] = output.len();
        output.push(copy_op(&program[index]));
        index += 1;
    }

    target_map[len] = output.len();

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= len => {
                *target = target_map[*target];
            }
            _ => {}
        }
    }

    (output, changed)
}

fn fold_binary(left: i64, right: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div if right != 0 => Some(left.wrapping_div(right)),
        Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
        _ => None,
    }
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
