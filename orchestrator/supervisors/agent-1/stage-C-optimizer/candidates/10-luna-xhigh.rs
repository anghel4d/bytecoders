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

fn valid_jumps(program: &[Op]) -> bool {
    let length = program.len();

    program.iter().all(|op| match op {
        Op::Jmp(target) | Op::Jz(target) => *target < length,
        _ => true,
    })
}

fn folded_at(program: &[Op], index: usize) -> Option<(Op, usize)> {
    if index + 2 < program.len() {
        match (&program[index], &program[index + 1], &program[index + 2]) {
            (Op::Push(a), Op::Push(b), Op::Add) => {
                Some((Op::Push(a.wrapping_add(*b)), 3))
            }
            (Op::Push(a), Op::Push(b), Op::Sub) => {
                Some((Op::Push(a.wrapping_sub(*b)), 3))
            }
            (Op::Push(a), Op::Push(b), Op::Mul) => {
                Some((Op::Push(a.wrapping_mul(*b)), 3))
            }
            (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                let value = if *a == i64::MIN && *b == -1 {
                    i64::MIN
                } else {
                    *a / *b
                };
                Some((Op::Push(value), 3))
            }
            (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                let value = if *a == i64::MIN && *b == -1 {
                    0
                } else {
                    *a % *b
                };
                Some((Op::Push(value), 3))
            }
            _ => None,
        }
    } else if index + 1 < program.len() {
        match (&program[index], &program[index + 1]) {
            (Op::Push(value), Op::Neg) => Some((Op::Push(value.wrapping_neg()), 2)),
            _ => None,
        }
    } else {
        None
    }
}

fn rewrite_at(
    program: &[Op],
    start: usize,
    removed: usize,
    replacement: Option<Op>,
) -> Vec<Op> {
    let mut output = Vec::with_capacity(
        program.len() - removed + usize::from(replacement.is_some()),
    );
    let mut mapping = vec![0; program.len()];
    let mut index = 0;

    while index < program.len() {
        if index == start {
            let destination = output.len();
            mapping[index] = destination;

            if let Some(op) = replacement {
                output.push(op);
            }

            for removed_index in 1..removed {
                mapping[index + removed_index] = destination;
            }

            index += removed;
        } else {
            mapping[index] = output.len();
            output.push(copy_op(&program[index]));
            index += 1;
        }
    }

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => *target = mapping[*target],
            _ => {}
        }
    }

    output
}

fn optimize_once(program: &[Op]) -> Option<Vec<Op>> {
    let mut targets = vec![false; program.len()];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => targets[*target] = true,
            _ => {}
        }
    }

    for index in 0..program.len() {
        if let Some((replacement, removed)) = folded_at(program, index) {
            if (index + 1..index + removed).all(|target| !targets[target]) {
                return Some(rewrite_at(
                    program,
                    index,
                    removed,
                    Some(replacement),
                ));
            }
        }

        if index + 1 < program.len()
            && matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop))
            && !targets[index]
            && !targets[index + 1]
        {
            return Some(rewrite_at(program, index, 2, None));
        }
    }

    None
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    if !valid_jumps(program) {
        return program.iter().map(copy_op).collect();
    }

    let mut optimized = program.iter().map(copy_op).collect();

    while let Some(next) = optimize_once(&optimized) {
        optimized = next;
    }

    optimized
}
