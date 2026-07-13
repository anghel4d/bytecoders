pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = clone_program(program);

    loop {
        let (next, changed) = optimize_pass(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return (clone_program(program), false);
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut old_to_new = vec![0usize; length];
    let mut optimized = Vec::with_capacity(length);
    let mut changed = false;
    let mut index = 0usize;

    while index < length {
        if let Some((removed, replacement)) =
            replacement_at(program, index, &targeted)
        {
            let new_index = optimized.len();
            for old_index in index..index + removed {
                old_to_new[old_index] = new_index;
            }
            optimized.push(replacement);
            changed = true;
            index += removed;
        } else {
            old_to_new[index] = optimized.len();
            optimized.push(clone_op(&program[index]));
            index += 1;
        }
    }

    if changed {
        for op in &mut optimized {
            match op {
                Op::Jmp(target) | Op::Jz(target) => {
                    let old_target = *target;
                    *target = old_to_new[old_target];
                }
                _ => {}
            }
        }
    }

    (optimized, changed)
}

fn replacement_at(
    program: &[Op],
    index: usize,
    targeted: &[bool],
) -> Option<(usize, Op)> {
    let remaining = program.len() - index;

    if remaining >= 3 && !targeted[index + 1] && !targeted[index + 2] {
        match (&program[index], &program[index + 1], &program[index + 2]) {
            (Op::Push(a), Op::Push(b), Op::Add) => {
                return Some((3, Op::Push((*a).wrapping_add(*b))));
            }
            (Op::Push(a), Op::Push(b), Op::Sub) => {
                return Some((3, Op::Push((*a).wrapping_sub(*b))));
            }
            (Op::Push(a), Op::Push(b), Op::Mul) => {
                return Some((3, Op::Push((*a).wrapping_mul(*b))));
            }
            (Op::Push(a), Op::Push(b), Op::Div) => {
                if *b != 0 {
                    return Some((3, Op::Push(peephole_div(*a, *b))));
                }
            }
            (Op::Push(a), Op::Push(b), Op::Mod) => {
                if *b != 0 {
                    return Some((3, Op::Push(peephole_mod(*a, *b))));
                }
            }
            _ => {}
        }
    }

    if remaining >= 2 && !targeted[index + 1] {
        match (&program[index], &program[index + 1]) {
            (Op::Push(value), Op::Neg) => {
                return Some((2, Op::Push((*value).wrapping_neg())));
            }
            (Op::Push(_), Op::Pop) => {
                return Some((2, Op::Push(0)));
            }
            _ => {}
        }
    }

    None
}

fn peephole_div(lhs: i64, rhs: i64) -> i64 {
    if lhs == i64::MIN && rhs == -1 {
        i64::MIN
    } else {
        lhs / rhs
    }
}

fn peephole_mod(lhs: i64, rhs: i64) -> i64 {
    if lhs == i64::MIN && rhs == -1 {
        0
    } else {
        lhs % rhs
    }
}

fn clone_program(program: &[Op]) -> Vec<Op> {
    let mut cloned = Vec::with_capacity(program.len());
    for op in program {
        cloned.push(clone_op(op));
    }
    cloned
}

fn clone_op(op: &Op) -> Op {
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
