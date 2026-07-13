pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = Vec::with_capacity(program.len());
    for op in program {
        current.push(copy_op(op));
    }

    loop {
        let next = optimize_pass(&current);
        if next.len() == current.len() {
            return next;
        }
        current = next;
    }
}

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targets = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < length => {
                targets[*target] = true;
            }
            _ => {}
        }
    }

    let mut old_to_new = vec![None; length];
    let mut optimized = Vec::with_capacity(length);
    let mut index = 0;

    while index < length {
        if length - index >= 3
            && !targets[index + 1]
            && !targets[index + 2]
        {
            if let Some(value) = fold_three(program, index) {
                old_to_new[index] = Some(optimized.len());
                optimized.push(Op::Push(value));
                index += 3;
                continue;
            }
        }

        if length - index >= 2
            && !targets[index + 1]
            && matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop))
        {
            old_to_new[index] = Some(optimized.len());
            index += 2;
            continue;
        }

        old_to_new[index] = Some(optimized.len());
        optimized.push(copy_op(&program[index]));
        index += 1;
    }

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                if old_target < length {
                    if let Some(new_target) = old_to_new[old_target] {
                        *target = new_target;
                    }
                }
            }
            _ => {}
        }
    }

    optimized
}

fn fold_three(program: &[Op], index: usize) -> Option<i64> {
    match (&program[index], &program[index + 1], &program[index + 2]) {
        (Op::Push(a), Op::Push(b), Op::Add) => Some((*a).wrapping_add(*b)),
        (Op::Push(a), Op::Push(b), Op::Sub) => Some((*a).wrapping_sub(*b)),
        (Op::Push(a), Op::Push(b), Op::Mul) => Some((*a).wrapping_mul(*b)),
        (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
            Some(fold_div(*a, *b))
        }
        (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
            Some(fold_rem(*a, *b))
        }
        _ => None,
    }
}

fn fold_div(a: i64, b: i64) -> i64 {
    if a == i64::MIN && b == -1 {
        i64::MIN
    } else {
        a / b
    }
}

fn fold_rem(a: i64, b: i64) -> i64 {
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
