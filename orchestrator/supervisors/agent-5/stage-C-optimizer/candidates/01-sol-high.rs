pub fn optimize(program: &[Op]) -> Vec<Op> {
    let (mut optimized, mut changed) = optimize_once(program);

    while changed {
        let result = optimize_once(&optimized);
        optimized = result.0;
        changed = result.1;
    }

    optimized
}

fn optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
    let len = program.len();
    let mut targeted = vec![false; len + 1];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => *target,
            _ => continue,
        };

        if target > len {
            return (program.iter().map(copy_op).collect(), false);
        }

        targeted[target] = true;
    }

    let mut optimized = Vec::with_capacity(len);
    let mut index_map = vec![0; len + 1];
    let mut changed = false;
    let mut i = 0;

    while i < len {
        let new_index = optimized.len();

        if i + 2 < len && !targeted[i + 1] && !targeted[i + 2] {
            if let (Op::Push(a), Op::Push(b)) = (&program[i], &program[i + 1]) {
                let folded = match &program[i + 2] {
                    Op::Add => Some(a.wrapping_add(*b)),
                    Op::Sub => Some(a.wrapping_sub(*b)),
                    Op::Mul => Some(a.wrapping_mul(*b)),
                    Op::Div if *b != 0 => Some(a.wrapping_div(*b)),
                    Op::Mod if *b != 0 => Some(a.wrapping_rem(*b)),
                    _ => None,
                };

                if let Some(value) = folded {
                    index_map[i] = new_index;
                    index_map[i + 1] = new_index;
                    index_map[i + 2] = new_index;
                    optimized.push(Op::Push(value));
                    changed = true;
                    i += 3;
                    continue;
                }
            }
        }

        if i + 1 < len && !targeted[i + 1] {
            match (&program[i], &program[i + 1]) {
                (Op::Push(value), Op::Neg) => {
                    index_map[i] = new_index;
                    index_map[i + 1] = new_index;
                    optimized.push(Op::Push(value.wrapping_neg()));
                    changed = true;
                    i += 2;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    index_map[i] = new_index;
                    index_map[i + 1] = new_index;
                    changed = true;
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        index_map[i] = new_index;
        optimized.push(copy_op(&program[i]));
        i += 1;
    }

    index_map[len] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = index_map[*target];
            }
            _ => {}
        }
    }

    (optimized, changed)
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
