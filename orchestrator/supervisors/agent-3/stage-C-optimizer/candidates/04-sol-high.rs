pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = copy_program(program);

    loop {
        let next = optimize_pass(&current);
        if next.len() == current.len() {
            return next;
        }
        current = next;
    }
}

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let len = program.len();
    let mut targeted = vec![false; len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= len {
                    return copy_program(program);
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(len);
    let mut old_to_new = vec![0; len + 1];
    let mut i = 0;

    while i < len {
        let output_index = output.len();

        let folded_binary = if i + 2 < len && !targeted[i + 1] && !targeted[i + 2] {
            match (&program[i], &program[i + 1], &program[i + 2]) {
                (Op::Push(a), Op::Push(b), Op::Add) => Some(a.wrapping_add(*b)),
                (Op::Push(a), Op::Push(b), Op::Sub) => Some(a.wrapping_sub(*b)),
                (Op::Push(a), Op::Push(b), Op::Mul) => Some(a.wrapping_mul(*b)),
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some(a.wrapping_div(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some(a.wrapping_rem(*b))
                }
                _ => None,
            }
        } else {
            None
        };

        if let Some(value) = folded_binary {
            old_to_new[i] = output_index;
            old_to_new[i + 1] = output_index;
            old_to_new[i + 2] = output_index;
            output.push(Op::Push(value));
            i += 3;
            continue;
        }

        if i + 1 < len && !targeted[i + 1] {
            if let (Op::Push(value), Op::Neg) = (&program[i], &program[i + 1]) {
                old_to_new[i] = output_index;
                old_to_new[i + 1] = output_index;
                output.push(Op::Push(value.wrapping_neg()));
                i += 2;
                continue;
            }
        }

        if i + 1 < len && !targeted[i] && !targeted[i + 1] {
            if let (Op::Push(_), Op::Pop) = (&program[i], &program[i + 1]) {
                old_to_new[i] = output_index;
                old_to_new[i + 1] = output_index;
                i += 2;
                continue;
            }
        }

        old_to_new[i] = output_index;
        output.push(copy_op(&program[i]));
        i += 1;
    }

    old_to_new[len] = output.len();

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = old_to_new[*target];
            }
            _ => {}
        }
    }

    output
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
