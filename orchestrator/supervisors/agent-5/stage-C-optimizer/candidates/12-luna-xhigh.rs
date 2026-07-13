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

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return program.iter().map(copy_op).collect();
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut mapping = vec![0; length];
    let mut optimized = Vec::with_capacity(length);
    let mut index = 0;

    while index < length {
        let mut replacement: Option<(usize, Op)> = None;

        if length - index >= 3 {
            match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(a), Op::Push(b), Op::Add) => {
                    replacement = Some((3, Op::Push((*a).wrapping_add(*b))));
                }
                (Op::Push(a), Op::Push(b), Op::Sub) => {
                    replacement = Some((3, Op::Push((*a).wrapping_sub(*b))));
                }
                (Op::Push(a), Op::Push(b), Op::Mul) => {
                    replacement = Some((3, Op::Push((*a).wrapping_mul(*b))));
                }
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    replacement = Some((3, Op::Push((*a).wrapping_div(*b))));
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    replacement = Some((3, Op::Push((*a).wrapping_rem(*b))));
                }
                _ => {}
            }
        }

        if replacement.is_none() && length - index >= 2 {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    replacement = Some((2, Op::Push((*value).wrapping_neg())));
                }
                (Op::Push(_), Op::Pop) if !(index + 2 == length && targeted[index]) => {
                    replacement = Some((2, Op::Push(0)));
                }
                _ => {}
            }
        }

        if let Some((span, op)) = replacement {
            let end = index + span;
            let mut safe = true;

            for target in (index + 1)..end {
                if targeted[target] {
                    safe = false;
                    break;
                }
            }

            if safe {
                let output_index = optimized.len();
                for source_index in index..end {
                    mapping[source_index] = output_index;
                }
                if matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop)) {
                    optimized.pop();
                }
                optimized.push(op);
                index = end;
                continue;
            }
        }

        mapping[index] = optimized.len();
        optimized.push(copy_op(&program[index]));
        index += 1;
    }

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = mapping[*target];
            }
            _ => {}
        }
    }

    optimized
}
