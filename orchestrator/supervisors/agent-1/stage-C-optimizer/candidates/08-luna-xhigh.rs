pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current: Vec<Op> = program.iter().map(copy_op).collect();

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

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target >= len => {
                return (program.iter().map(copy_op).collect(), false);
            }
            _ => {}
        }
    }

    let mut targeted = vec![false; len];
    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(len);
    let mut remap = vec![0; len + 1];
    let mut changed = false;
    let mut index = 0;

    while index < len {
        if index + 2 < len
            && !targeted[index]
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            if let Some(folded) =
                fold_three(&program[index], &program[index + 1], &program[index + 2])
            {
                let position = output.len();
                remap[index] = position;
                remap[index + 1] = position;
                remap[index + 2] = position;
                output.push(folded);
                index += 3;
                changed = true;
                continue;
            }
        }

        if index + 1 < len && !targeted[index] && !targeted[index + 1] {
            if matches!(
                (&program[index], &program[index + 1]),
                (&Op::Push(_), &Op::Pop)
            ) {
                let position = output.len();
                remap[index] = position;
                remap[index + 1] = position;
                index += 2;
                changed = true;
                continue;
            }

            if let Some(folded) = fold_two(&program[index], &program[index + 1]) {
                let position = output.len();
                remap[index] = position;
                remap[index + 1] = position;
                output.push(folded);
                index += 2;
                changed = true;
                continue;
            }
        }

        remap[index] = output.len();
        output.push(copy_op(&program[index]));
        index += 1;
    }

    remap[len] = output.len();

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = remap[*target];
            }
            _ => {}
        }
    }

    (output, changed)
}

fn fold_three(first: &Op, second: &Op, third: &Op) -> Option<Op> {
    match (first, second, third) {
        (&Op::Push(a), &Op::Push(b), &Op::Add) => Some(Op::Push(a.wrapping_add(b))),
        (&Op::Push(a), &Op::Push(b), &Op::Sub) => Some(Op::Push(a.wrapping_sub(b))),
        (&Op::Push(a), &Op::Push(b), &Op::Mul) => Some(Op::Push(a.wrapping_mul(b))),
        (&Op::Push(a), &Op::Push(b), &Op::Div) if b != 0 => {
            Some(Op::Push(wrapping_div(a, b)))
        }
        (&Op::Push(a), &Op::Push(b), &Op::Mod) if b != 0 => {
            Some(Op::Push(wrapping_rem(a, b)))
        }
        _ => None,
    }
}

fn fold_two(first: &Op, second: &Op) -> Option<Op> {
    match (first, second) {
        (&Op::Push(value), &Op::Neg) => Some(Op::Push(value.wrapping_neg())),
        _ => None,
    }
}

fn wrapping_div(a: i64, b: i64) -> i64 {
    if a == i64::MIN && b == -1 {
        i64::MIN
    } else {
        a / b
    }
}

fn wrapping_rem(a: i64, b: i64) -> i64 {
    if a == i64::MIN && b == -1 {
        0
    } else {
        a % b
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
