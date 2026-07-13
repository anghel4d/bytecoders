fn optimize(program: &[Op]) -> Vec<Op> {
    let (mut optimized, mut changed) = optimize_pass(program);

    while changed {
        let (next, next_changed) = optimize_pass(&optimized);
        optimized = next;
        changed = next_changed;
    }

    optimized
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < length {
                    targeted[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut mapping = vec![0; length];
    let mut changed = false;
    let mut index = 0;

    while index < length {
        if index + 2 < length && !targeted[index + 1] && !targeted[index + 2] {
            match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(a), Op::Push(b), Op::Add) => {
                    let output_index = output.len();
                    output.push(Op::Push((*a).wrapping_add(*b)));
                    mapping[index] = output_index;
                    mapping[index + 1] = output_index;
                    mapping[index + 2] = output_index;
                    index += 3;
                    changed = true;
                    continue;
                }
                (Op::Push(a), Op::Push(b), Op::Sub) => {
                    let output_index = output.len();
                    output.push(Op::Push((*a).wrapping_sub(*b)));
                    mapping[index] = output_index;
                    mapping[index + 1] = output_index;
                    mapping[index + 2] = output_index;
                    index += 3;
                    changed = true;
                    continue;
                }
                (Op::Push(a), Op::Push(b), Op::Mul) => {
                    let output_index = output.len();
                    output.push(Op::Push((*a).wrapping_mul(*b)));
                    mapping[index] = output_index;
                    mapping[index + 1] = output_index;
                    mapping[index + 2] = output_index;
                    index += 3;
                    changed = true;
                    continue;
                }
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    let output_index = output.len();
                    output.push(Op::Push((*a).wrapping_div(*b)));
                    mapping[index] = output_index;
                    mapping[index + 1] = output_index;
                    mapping[index + 2] = output_index;
                    index += 3;
                    changed = true;
                    continue;
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    let output_index = output.len();
                    output.push(Op::Push((*a).wrapping_rem(*b)));
                    mapping[index] = output_index;
                    mapping[index + 1] = output_index;
                    mapping[index + 2] = output_index;
                    index += 3;
                    changed = true;
                    continue;
                }
                _ => {}
            }
        }

        if index + 1 < length && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    let output_index = output.len();
                    output.push(Op::Push((*value).wrapping_neg()));
                    mapping[index] = output_index;
                    mapping[index + 1] = output_index;
                    index += 2;
                    changed = true;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    let output_index = output.len();
                    mapping[index] = output_index;
                    mapping[index + 1] = output_index;
                    index += 2;
                    changed = true;
                    continue;
                }
                _ => {}
            }
        }

        mapping[index] = output.len();
        output.push(copy_op(&program[index]));
        index += 1;
    }

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < length {
                    *target = mapping[*target];
                }
            }
            _ => {}
        }
    }

    (output, changed)
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
