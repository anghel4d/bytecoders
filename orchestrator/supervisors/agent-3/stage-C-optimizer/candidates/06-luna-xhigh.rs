pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = copy_program(program);

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

    for instruction in program {
        match instruction {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= len {
                    return (copy_program(program), false);
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut old_to_new = vec![0usize; len + 1];
    let mut result = Vec::with_capacity(len);
    let mut changed = false;
    let mut index = 0;

    while index < len {
        old_to_new[index] = result.len();

        if index + 1 < len
            && !targeted[index + 1]
            && matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop))
        {
            old_to_new[index + 1] = result.len();
            index += 2;
            changed = true;
            continue;
        }

        if index + 1 < len
            && !targeted[index + 1]
            && matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Neg))
        {
            if let Op::Push(value) = &program[index] {
                let destination = result.len();
                old_to_new[index + 1] = destination;
                result.push(Op::Push(value.wrapping_neg()));
                index += 2;
                changed = true;
                continue;
            }
        }

        if index + 2 < len && !targeted[index + 1] && !targeted[index + 2] {
            if let Some(value) = folded_binary(program, index) {
                let destination = result.len();
                old_to_new[index + 1] = destination;
                old_to_new[index + 2] = destination;
                result.push(Op::Push(value));
                index += 3;
                changed = true;
                continue;
            }
        }

        result.push(copy_op(&program[index]));
        index += 1;
    }

    old_to_new[len] = result.len();

    for instruction in &mut result {
        match instruction {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = old_to_new[*target];
            }
            _ => {}
        }
    }

    (result, changed)
}

fn folded_binary(program: &[Op], index: usize) -> Option<i64> {
    match (&program[index], &program[index + 1], &program[index + 2]) {
        (Op::Push(a), Op::Push(b), Op::Add) => Some(a.wrapping_add(*b)),
        (Op::Push(a), Op::Push(b), Op::Sub) => Some(a.wrapping_sub(*b)),
        (Op::Push(a), Op::Push(b), Op::Mul) => Some(a.wrapping_mul(*b)),
        (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => Some(a.wrapping_div(*b)),
        (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => Some(a.wrapping_rem(*b)),
        _ => None,
    }
}

fn copy_program(program: &[Op]) -> Vec<Op> {
    let mut result = Vec::with_capacity(program.len());
    for instruction in program {
        result.push(copy_op(instruction));
    }
    result
}

fn copy_op(instruction: &Op) -> Op {
    match instruction {
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
