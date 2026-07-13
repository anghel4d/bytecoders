pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = opt_clone_program(program);

    loop {
        let (next, changed) = opt_pass(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}

fn opt_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let n = program.len();
    let mut targeted = vec![false; n];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= n {
                    return (opt_clone_program(program), false);
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(n);
    let mut old_to_new = vec![usize::MAX; n + 1];
    let mut changed = false;
    let mut i = 0;

    while i < n {
        if i + 1 < n {
            if let (Op::Push(_), Op::Pop) = (&program[i], &program[i + 1]) {
                if !targeted[i] && !targeted[i + 1] {
                    i += 2;
                    changed = true;
                    continue;
                }
            }

            if let (Op::Push(value), Op::Neg) = (&program[i], &program[i + 1]) {
                if !targeted[i + 1] {
                    old_to_new[i] = output.len();
                    output.push(Op::Push((*value).wrapping_neg()));
                    i += 2;
                    changed = true;
                    continue;
                }
            }
        }

        if i + 2 < n && !targeted[i + 1] && !targeted[i + 2] {
            let folded = match (&program[i], &program[i + 1], &program[i + 2]) {
                (Op::Push(left), Op::Push(right), Op::Add) => {
                    Some((*left).wrapping_add(*right))
                }
                (Op::Push(left), Op::Push(right), Op::Sub) => {
                    Some((*left).wrapping_sub(*right))
                }
                (Op::Push(left), Op::Push(right), Op::Mul) => {
                    Some((*left).wrapping_mul(*right))
                }
                (Op::Push(left), Op::Push(right), Op::Div) if *right != 0 => {
                    Some((*left).wrapping_div(*right))
                }
                (Op::Push(left), Op::Push(right), Op::Mod) if *right != 0 => {
                    Some((*left).wrapping_rem(*right))
                }
                _ => None,
            };

            if let Some(value) = folded {
                old_to_new[i] = output.len();
                output.push(Op::Push(value));
                i += 3;
                changed = true;
                continue;
            }
        }

        old_to_new[i] = output.len();
        output.push(opt_clone_op(&program[i]));
        i += 1;
    }

    old_to_new[n] = output.len();

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                let mapped = old_to_new[*target];
                if mapped == usize::MAX {
                    return (opt_clone_program(program), false);
                }
                *target = mapped;
            }
            _ => {}
        }
    }

    (output, changed)
}

fn opt_clone_program(program: &[Op]) -> Vec<Op> {
    let mut output = Vec::with_capacity(program.len());
    for op in program {
        output.push(opt_clone_op(op));
    }
    output
}

fn opt_clone_op(op: &Op) -> Op {
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
