fn ano_copy_op(op: &Op) -> Op {
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

fn ano_copy_program(program: &[Op]) -> Vec<Op> {
    let mut result = Vec::with_capacity(program.len());
    for op in program {
        result.push(ano_copy_op(op));
    }
    result
}

fn ano_fold_binary(op: &Op, left: i64, right: i64) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div if right != 0 => Some(left.wrapping_div(right)),
        Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
        _ => None,
    }
}

fn ano_optimize_block(block: &[Op], keep_nonempty: bool) -> Vec<Op> {
    let mut current = ano_copy_program(block);

    loop {
        let mut next = Vec::with_capacity(current.len());
        let mut changed = false;
        let mut index = 0;

        while index < current.len() {
            if index + 1 < current.len() {
                match (&current[index], &current[index + 1]) {
                    (Op::Push(_), Op::Pop) => {
                        changed = true;
                        index += 2;
                        continue;
                    }
                    (Op::Push(value), Op::Neg) => {
                        next.push(Op::Push((*value).wrapping_neg()));
                        changed = true;
                        index += 2;
                        continue;
                    }
                    _ => {}
                }
            }

            if index + 2 < current.len() {
                if let (Op::Push(left), Op::Push(right)) =
                    (&current[index], &current[index + 1])
                {
                    if let Some(value) =
                        ano_fold_binary(&current[index + 2], *left, *right)
                    {
                        next.push(Op::Push(value));
                        changed = true;
                        index += 3;
                        continue;
                    }
                }
            }

            next.push(ano_copy_op(&current[index]));
            index += 1;
        }

        if !changed {
            if keep_nonempty && current.is_empty() {
                return ano_copy_program(block);
            }
            return current;
        }

        current = next;
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();

    if length == 0 {
        return Vec::new();
    }

    let mut leaders = vec![false; length];
    let mut jump_targets = vec![false; length];
    leaders[0] = true;

    for (index, op) in program.iter().enumerate() {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return ano_copy_program(program);
                }

                leaders[*target] = true;
                jump_targets[*target] = true;

                if index + 1 < length {
                    leaders[index + 1] = true;
                }
            }
            _ => {}
        }
    }

    let mut block_starts = Vec::new();
    for index in 0..length {
        if leaders[index] {
            block_starts.push(index);
        }
    }
    block_starts.push(length);

    let mut target_map = vec![0; length];
    let mut result = Vec::new();

    for block_index in 0..block_starts.len() - 1 {
        let start = block_starts[block_index];
        let end = block_starts[block_index + 1];

        target_map[start] = result.len();

        let block = ano_optimize_block(&program[start..end], jump_targets[start]);
        result.extend(block);
    }

    for op in &mut result {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = target_map[*target];
            }
            _ => {}
        }
    }

    result
}
