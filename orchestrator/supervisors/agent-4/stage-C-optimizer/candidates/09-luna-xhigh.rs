pub fn optimize(program: &[Op]) -> Vec<Op> {
    let (mut optimized, mut changed) = optimize_once(program);
    while changed {
        let (next, next_changed) = optimize_once(&optimized);
        optimized = next;
        changed = next_changed;
    }
    optimized
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

fn optimize_targeted(program: &[Op]) -> Vec<bool> {
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
    targeted
}

fn optimize_fold(
    program: &[Op],
    targeted: &[bool],
    index: usize,
) -> Option<(Option<Op>, usize)> {
    if index + 2 < program.len() && !targeted[index + 1] && !targeted[index + 2] {
        match (&program[index], &program[index + 1], &program[index + 2]) {
            (Op::Push(a), Op::Push(b), Op::Add) => {
                return Some((Some(Op::Push((*a).wrapping_add(*b))), 3));
            }
            (Op::Push(a), Op::Push(b), Op::Sub) => {
                return Some((Some(Op::Push((*a).wrapping_sub(*b))), 3));
            }
            (Op::Push(a), Op::Push(b), Op::Mul) => {
                return Some((Some(Op::Push((*a).wrapping_mul(*b))), 3));
            }
            (Op::Push(a), Op::Push(b), Op::Div) => {
                return (*a)
                    .checked_div(*b)
                    .map(|value| (Some(Op::Push(value)), 3));
            }
            (Op::Push(a), Op::Push(b), Op::Mod) => {
                return (*a)
                    .checked_rem(*b)
                    .map(|value| (Some(Op::Push(value)), 3));
            }
            _ => {}
        }
    }

    if index + 1 < program.len() && !targeted[index + 1] {
        if let (Op::Push(value), Op::Neg) = (&program[index], &program[index + 1]) {
            return Some((Some(Op::Push((*value).wrapping_neg())), 2));
        }
    }

    if index + 1 < program.len()
        && !targeted[index]
        && !targeted[index + 1]
        && matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop))
    {
        return Some((None, 2));
    }

    None
}

fn optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
    let targeted = optimize_targeted(program);
    let mut targets = vec![None; program.len()];
    let mut output = Vec::with_capacity(program.len());
    let mut changed = false;
    let mut index = 0;

    while index < program.len() {
        if let Some((replacement, consumed)) = optimize_fold(program, &targeted, index) {
            changed = true;
            if let Some(op) = replacement {
                targets[index] = Some(output.len());
                output.push(op);
            }
            index += consumed;
        } else {
            targets[index] = Some(output.len());
            output.push(optimize_copy_op(&program[index]));
            index += 1;
        }
    }

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < targets.len() {
                    if let Some(new_target) = targets[*target] {
                        *target = new_target;
                    }
                }
            }
            _ => {}
        }
    }

    (output, changed)
}
