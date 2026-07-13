pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = copy_program(program);

    loop {
        let (next, changed) = optimize_once(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}

fn optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
    let len = program.len();
    let mut targeted = vec![false; len + 1];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target > len {
                    return (copy_program(program), false);
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(len);
    let mut old_to_new = vec![0usize; len + 1];
    let mut jumps = Vec::new();
    let mut changed = false;
    let mut i = 0;

    while i < len {
        if i + 2 < len && !targeted[i + 1] && !targeted[i + 2] {
            if let (Op::Push(a), Op::Push(b)) = (&program[i], &program[i + 1]) {
                if let Some(value) = fold_binary(*a, *b, &program[i + 2]) {
                    let new_index = output.len();
                    old_to_new[i] = new_index;
                    old_to_new[i + 1] = new_index;
                    old_to_new[i + 2] = new_index;
                    output.push(Op::Push(value));
                    changed = true;
                    i += 3;
                    continue;
                }
            }
        }

        if i + 1 < len && !targeted[i + 1] {
            match (&program[i], &program[i + 1]) {
                (Op::Push(value), Op::Neg) => {
                    let new_index = output.len();
                    old_to_new[i] = new_index;
                    old_to_new[i + 1] = new_index;
                    output.push(Op::Push(value.wrapping_neg()));
                    changed = true;
                    i += 2;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    let new_index = output.len();
                    old_to_new[i] = new_index;
                    old_to_new[i + 1] = new_index;
                    changed = true;
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        old_to_new[i] = output.len();

        match &program[i] {
            Op::Jmp(target) => {
                jumps.push((output.len(), *target));
                output.push(Op::Jmp(*target));
            }
            Op::Jz(target) => {
                jumps.push((output.len(), *target));
                output.push(Op::Jz(*target));
            }
            op => output.push(copy_op(op)),
        }

        i += 1;
    }

    old_to_new[len] = output.len();

    for (position, old_target) in jumps {
        let new_target = old_to_new[old_target];
        match &mut output[position] {
            Op::Jmp(target) | Op::Jz(target) => *target = new_target,
            _ => unreachable!(),
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

fn copy_program(program: &[Op]) -> Vec<Op> {
    program.iter().map(copy_op).collect()
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
