pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = Vec::with_capacity(program.len());
    for op in program {
        current.push(optimize_clone_op(op));
    }

    loop {
        let next = optimize_pass(&current);
        if next.len() == current.len() {
            return next;
        }
        current = next;
    }
}

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

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let mut targeted = vec![false; program.len()];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < targeted.len() {
                    targeted[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut mapping = vec![0; program.len()];
    let mut result = Vec::with_capacity(program.len());
    let mut index = 0;

    while index < program.len() {
        if index + 2 < program.len()
            && !targeted[index]
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            if let (Op::Push(a), Op::Push(b)) = (&program[index], &program[index + 1]) {
                if let Some(value) =
                    optimize_fold_binary(*a, *b, &program[index + 2])
                {
                    let position = result.len();
                    mapping[index] = position;
                    mapping[index + 1] = position;
                    mapping[index + 2] = position;
                    result.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < program.len()
            && !targeted[index]
            && !targeted[index + 1]
        {
            match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) => {
                    let position = result.len();
                    mapping[index] = position;
                    mapping[index + 1] = position;
                    index += 2;
                    continue;
                }
                (Op::Push(value), Op::Neg) => {
                    let position = result.len();
                    mapping[index] = position;
                    mapping[index + 1] = position;
                    result.push(Op::Push(value.wrapping_neg()));
                    index += 2;
                    continue;
                }
                _ => {}
            }
        }

        mapping[index] = result.len();
        result.push(optimize_clone_op(&program[index]));
        index += 1;
    }

    for op in &mut result {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < mapping.len() {
                    *target = mapping[*target];
                }
            }
            _ => {}
        }
    }

    result
}

fn optimize_fold_binary(a: i64, b: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(a.wrapping_add(b)),
        Op::Sub => Some(a.wrapping_sub(b)),
        Op::Mul => Some(a.wrapping_mul(b)),
        Op::Div => {
            if b == 0 {
                None
            } else if a == i64::MIN && b == -1 {
                Some(i64::MIN)
            } else {
                Some(a / b)
            }
        }
        Op::Mod => {
            if b == 0 {
                None
            } else if a == i64::MIN && b == -1 {
                Some(0)
            } else {
                Some(a % b)
            }
        }
        _ => None,
    }
}
