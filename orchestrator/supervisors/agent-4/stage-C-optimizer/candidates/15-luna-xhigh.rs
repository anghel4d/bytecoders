fn optimize_clone_op(op: &Op) -> Op {
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

fn optimize_fold_binary(a: i64, b: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(a.wrapping_add(b)),
        Op::Sub => Some(a.wrapping_sub(b)),
        Op::Mul => Some(a.wrapping_mul(b)),
        Op::Div if b != 0 => Some(a.wrapping_div(b)),
        Op::Mod if b != 0 => Some(a.wrapping_rem(b)),
        _ => None,
    }
}

fn optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return (program.iter().map(optimize_clone_op).collect(), false);
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut old_to_new = vec![0; length];
    let mut changed = false;
    let mut index = 0;

    while index < length {
        if length - index >= 3 {
            if let (Op::Push(a), Op::Push(b)) = (&program[index], &program[index + 1]) {
                if let Some(value) =
                    optimize_fold_binary(*a, *b, &program[index + 2])
                {
                    if !targeted[index + 1] && !targeted[index + 2] {
                        let destination = output.len();
                        old_to_new[index] = destination;
                        old_to_new[index + 1] = destination;
                        old_to_new[index + 2] = destination;
                        output.push(Op::Push(value));
                        index += 3;
                        changed = true;
                        continue;
                    }
                }
            }
        }

        if length - index >= 2 {
            if let (Op::Push(value), Op::Neg) = (&program[index], &program[index + 1]) {
                if !targeted[index + 1] {
                    let destination = output.len();
                    old_to_new[index] = destination;
                    old_to_new[index + 1] = destination;
                    output.push(Op::Push((*value).wrapping_neg()));
                    index += 2;
                    changed = true;
                    continue;
                }
            }

            if let (Op::Push(_), Op::Pop) = (&program[index], &program[index + 1]) {
                if !targeted[index] && !targeted[index + 1] {
                    let destination = output.len();
                    old_to_new[index] = destination;
                    old_to_new[index + 1] = destination;
                    index += 2;
                    changed = true;
                    continue;
                }
            }
        }

        old_to_new[index] = output.len();
        output.push(optimize_clone_op(&program[index]));
        index += 1;
    }

    let mut rewritten = Vec::with_capacity(output.len());

    for op in output {
        match op {
            Op::Jmp(target) => rewritten.push(Op::Jmp(old_to_new[target])),
            Op::Jz(target) => rewritten.push(Op::Jz(old_to_new[target])),
            op => rewritten.push(op),
        }
    }

    (rewritten, changed)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = program.iter().map(optimize_clone_op).collect();

    loop {
        let (next, changed) = optimize_once(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}
