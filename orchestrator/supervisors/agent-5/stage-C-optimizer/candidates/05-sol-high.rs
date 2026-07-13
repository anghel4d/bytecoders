pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current: Vec<Op> = program.iter().map(copy_op).collect();

    loop {
        let (next, changed) = optimize_pass(&current);
        if !changed {
            return current;
        }
        current = next;
    }
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

    let mut index_map = vec![0usize; len + 1];
    let mut output = Vec::with_capacity(len);
    let mut changed = false;
    let mut index = 0;

    while index < len {
        let output_index = output.len();
        index_map[index] = output_index;

        if index + 2 < len && !targeted[index + 1] && !targeted[index + 2] {
            if let (Op::Push(a), Op::Push(b)) = (&program[index], &program[index + 1]) {
                if let Some(value) = fold_binary(*a, *b, &program[index + 2]) {
                    index_map[index + 1] = output_index;
                    index_map[index + 2] = output_index;
                    output.push(Op::Push(value));
                    index += 3;
                    changed = true;
                    continue;
                }
            }
        }

        if index + 1 < len && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    index_map[index + 1] = output_index;
                    output.push(Op::Push(value.wrapping_neg()));
                    index += 2;
                    changed = true;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    index_map[index + 1] = output_index;
                    index += 2;
                    changed = true;
                    continue;
                }
                _ => {}
            }
        }

        output.push(copy_op(&program[index]));
        index += 1;
    }

    index_map[len] = output.len();

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= len => {
                *target = index_map[*target];
            }
            _ => {}
        }
    }

    (output, changed)
}

fn fold_binary(a: i64, b: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(a.wrapping_add(b)),
        Op::Sub => Some(a.wrapping_sub(b)),
        Op::Mul => Some(a.wrapping_mul(b)),
        Op::Div if b != 0 => Some(a.wrapping_div(b)),
        Op::Mod if b != 0 => Some(a.wrapping_rem(b)),
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
