pub fn optimize(program: &[Op]) -> Vec<Op> {
    let (mut optimized, mut changed) = optimize_once(program);

    while changed {
        let (next, next_changed) = optimize_once(&optimized);
        optimized = next;
        changed = next_changed;
    }

    optimized
}

fn optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
    let len = program.len();

    if program.iter().any(|op| match op {
        Op::Jmp(target) | Op::Jz(target) => *target >= len,
        _ => false,
    }) {
        return (program.iter().map(copy_op).collect(), false);
    }

    let mut targeted = vec![false; len];
    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => targeted[*target] = true,
            _ => {}
        }
    }

    let mut index_map = vec![0; len + 1];
    let mut old_index = 0;
    let mut new_index = 0;
    let mut changed = false;

    while old_index < len {
        index_map[old_index] = new_index;

        if let Some((span, replacement)) = peephole(program, &targeted, old_index) {
            for offset in 1..span {
                index_map[old_index + offset] = new_index;
            }

            if replacement.is_some() {
                new_index += 1;
            }

            old_index += span;
            changed = true;
        } else {
            old_index += 1;
            new_index += 1;
        }
    }

    index_map[len] = new_index;

    let mut result = Vec::with_capacity(new_index);
    let mut index = 0;

    while index < len {
        if let Some((span, replacement)) = peephole(program, &targeted, index) {
            if let Some(op) = replacement {
                result.push(op);
            }
            index += span;
        } else {
            result.push(copy_op_remapping_jumps(&program[index], &index_map));
            index += 1;
        }
    }

    (result, changed)
}

fn peephole(program: &[Op], targeted: &[bool], index: usize) -> Option<(usize, Option<Op>)> {
    if index + 2 < program.len() && !targeted[index + 1] && !targeted[index + 2] {
        match (&program[index], &program[index + 1], &program[index + 2]) {
            (Op::Push(a), Op::Push(b), Op::Add) => {
                return Some((3, Some(Op::Push(a.wrapping_add(*b)))));
            }
            (Op::Push(a), Op::Push(b), Op::Sub) => {
                return Some((3, Some(Op::Push(a.wrapping_sub(*b)))));
            }
            (Op::Push(a), Op::Push(b), Op::Mul) => {
                return Some((3, Some(Op::Push(a.wrapping_mul(*b)))));
            }
            (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                return Some((3, Some(Op::Push(a.wrapping_div(*b)))));
            }
            (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                return Some((3, Some(Op::Push(a.wrapping_rem(*b)))));
            }
            _ => {}
        }
    }

    if index + 1 < program.len() && !targeted[index + 1] {
        match (&program[index], &program[index + 1]) {
            (Op::Push(_), Op::Pop) => return Some((2, None)),
            (Op::Push(value), Op::Neg) => {
                return Some((2, Some(Op::Push(value.wrapping_neg()))));
            }
            _ => {}
        }
    }

    None
}

fn copy_op_remapping_jumps(op: &Op, index_map: &[usize]) -> Op {
    match op {
        Op::Jmp(target) => Op::Jmp(index_map[*target]),
        Op::Jz(target) => Op::Jz(index_map[*target]),
        _ => copy_op(op),
    }
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
