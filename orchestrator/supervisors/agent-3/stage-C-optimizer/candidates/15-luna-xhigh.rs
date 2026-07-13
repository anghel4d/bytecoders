pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = optimize_pass(program);

    loop {
        let next = optimize_pass(&current);
        if next.len() == current.len() {
            return current;
        }
        current = next;
    }
}

fn optimize_pass(program: &[Op]) -> Vec<Op> {
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

    let mut lowered = Vec::with_capacity(len);
    let mut mapping = vec![None; len];
    let mut index = 0;

    while index < len {
        if !targeted[index] {
            if index + 1 < len && !targeted[index + 1] {
                if let (Op::Push(value), Op::Pop) = (&program[index], &program[index + 1]) {
                    index += 2;
                    continue;
                }

                if let (Op::Push(value), Op::Neg) =
                    (&program[index], &program[index + 1])
                {
                    lowered.push(Op::Push((*value).wrapping_neg()));
                    index += 2;
                    continue;
                }
            }

            if index + 2 < len
                && !targeted[index + 1]
                && !targeted[index + 2]
            {
                let folded = match (
                    &program[index],
                    &program[index + 1],
                    &program[index + 2],
                ) {
                    (Op::Push(a), Op::Push(b), Op::Add) => {
                        Some((*a).wrapping_add(*b))
                    }
                    (Op::Push(a), Op::Push(b), Op::Sub) => {
                        Some((*a).wrapping_sub(*b))
                    }
                    (Op::Push(a), Op::Push(b), Op::Mul) => {
                        Some((*a).wrapping_mul(*b))
                    }
                    (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                        Some((*a).wrapping_div(*b))
                    }
                    (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                        Some((*a).wrapping_rem(*b))
                    }
                    _ => None,
                };

                if let Some(value) = folded {
                    lowered.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        mapping[index] = Some(lowered.len());
        lowered.push(optimize_copy_op(&program[index]));
        index += 1;
    }

    let mut rewritten = Vec::with_capacity(lowered.len());

    for op in lowered {
        match op {
            Op::Jmp(target) => {
                rewritten.push(Op::Jmp(optimize_remap_target(target, &mapping)));
            }
            Op::Jz(target) => {
                rewritten.push(Op::Jz(optimize_remap_target(target, &mapping)));
            }
            op => rewritten.push(op),
        }
    }

    rewritten
}

fn optimize_copy_op(op: &Op) -> Op {
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

fn optimize_remap_target(target: usize, mapping: &[Option<usize>]) -> usize {
    match mapping.get(target) {
        Some(Some(mapped)) => *mapped,
        _ => target,
    }
}
