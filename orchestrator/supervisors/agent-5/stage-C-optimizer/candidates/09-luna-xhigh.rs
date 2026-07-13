fn clone_op(op: &Op) -> Op {
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

fn fold_at(program: &[Op], index: usize) -> Option<(usize, Op)> {
    if index + 3 <= program.len() {
        if let (Op::Push(a), Op::Push(b), Op::Add) =
            (&program[index], &program[index + 1], &program[index + 2])
        {
            return Some((3, Op::Push(a.wrapping_add(*b))));
        }
        if let (Op::Push(a), Op::Push(b), Op::Sub) =
            (&program[index], &program[index + 1], &program[index + 2])
        {
            return Some((3, Op::Push(a.wrapping_sub(*b))));
        }
        if let (Op::Push(a), Op::Push(b), Op::Mul) =
            (&program[index], &program[index + 1], &program[index + 2])
        {
            return Some((3, Op::Push(a.wrapping_mul(*b))));
        }
        if let (Op::Push(a), Op::Push(b), Op::Div) =
            (&program[index], &program[index + 1], &program[index + 2])
        {
            if *b != 0 {
                return Some((3, Op::Push(a.wrapping_div(*b))));
            }
        }
        if let (Op::Push(a), Op::Push(b), Op::Mod) =
            (&program[index], &program[index + 1], &program[index + 2])
        {
            if *b != 0 {
                return Some((3, Op::Push(a.wrapping_rem(*b))));
            }
        }
    }

    if index + 2 <= program.len() {
        if let (Op::Push(value), Op::Neg) = (&program[index], &program[index + 1]) {
            return Some((2, Op::Push(value.wrapping_neg())));
        }
        if let (Op::Push(_), Op::Pop) = (&program[index], &program[index + 1]) {
            return Some((2, Op::Halt));
        }
    }

    None
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length + 1];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= length => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut optimized = Vec::with_capacity(length);
    let mut mapping = vec![None; length + 1];
    let mut index = 0;
    let mut changed = false;

    while index < length {
        if let Some((width, replacement)) = fold_at(program, index) {
            let end = index + width;
            if (index..end).all(|position| !targeted[position]) {
                mapping[index] = Some(optimized.len());
                optimized.push(replacement);
                index = end;
                changed = true;
                continue;
            }
        }

        mapping[index] = Some(optimized.len());
        optimized.push(clone_op(&program[index]));
        index += 1;
    }

    mapping[length] = Some(optimized.len());

    let optimized_length = optimized.len();
    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= length => {
                if let Some(mapped) = mapping[*target] {
                    *target = mapped;
                } else if *target == length {
                    *target = optimized_length;
                }
            }
            _ => {}
        }
    }

    (optimized, changed)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = program.iter().map(clone_op).collect::<Vec<_>>();

    loop {
        let (next, changed) = optimize_pass(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}
