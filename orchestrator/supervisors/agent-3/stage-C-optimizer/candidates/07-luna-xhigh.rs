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

fn fold_constants(left: i64, right: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div if right != 0 => Some(left.wrapping_div(right)),
        Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
        _ => None,
    }
}

fn remap_target(
    target: usize,
    old_to_new: &[Option<usize>],
    old_len: usize,
    new_len: usize,
) -> usize {
    if target < old_len {
        old_to_new[target].unwrap_or(new_len)
    } else if target == old_len {
        new_len
    } else {
        new_len.saturating_add(target - old_len)
    }
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let old_len = program.len();
    let mut targeted = vec![false; old_len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < old_len => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(old_len);
    let mut old_to_new = vec![None; old_len];
    let mut changed = false;
    let mut index = 0;

    while index < old_len {
        if index <= old_len.saturating_sub(3)
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(left), Op::Push(right), op) => {
                    fold_constants(*left, *right, op)
                }
                _ => None,
            };

            if let Some(value) = folded {
                old_to_new[index] = Some(output.len());
                output.push(Op::Push(value));
                index += 3;
                changed = true;
                continue;
            }
        }

        if index <= old_len.saturating_sub(2) {
            if let (Op::Push(_), Op::Pop) = (&program[index], &program[index + 1]) {
                if !targeted[index] && !targeted[index + 1] {
                    index += 2;
                    changed = true;
                    continue;
                }
            }

            if !targeted[index + 1] {
                if let (Op::Push(value), Op::Neg) =
                    (&program[index], &program[index + 1])
                {
                    old_to_new[index] = Some(output.len());
                    output.push(Op::Push((*value).wrapping_neg()));
                    index += 2;
                    changed = true;
                    continue;
                }
            }
        }

        old_to_new[index] = Some(output.len());
        output.push(copy_op(&program[index]));
        index += 1;
    }

    if changed {
        let new_len = output.len();
        for op in &mut output {
            match op {
                Op::Jmp(target) | Op::Jz(target) => {
                    *target = remap_target(*target, &old_to_new, old_len, new_len);
                }
                _ => {}
            }
        }
    }

    (output, changed)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = program.iter().map(copy_op).collect();

    loop {
        let (next, changed) = optimize_pass(&current);
        if !changed {
            return current;
        }
        current = next;
    }
}
