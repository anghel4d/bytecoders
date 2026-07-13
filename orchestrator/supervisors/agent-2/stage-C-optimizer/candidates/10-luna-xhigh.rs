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

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let mut targeted = vec![false; program.len()];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < program.len() => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut optimized = Vec::with_capacity(program.len());
    let mut old_to_new = vec![0; program.len()];
    let mut index = 0;

    while index < program.len() {
        if index + 2 < program.len()
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            let folded = match (
                &program[index],
                &program[index + 1],
                &program[index + 2],
            ) {
                (Op::Push(a), Op::Push(b), Op::Add) => {
                    Some(Op::Push((*a).wrapping_add(*b)))
                }
                (Op::Push(a), Op::Push(b), Op::Sub) => {
                    Some(Op::Push((*a).wrapping_sub(*b)))
                }
                (Op::Push(a), Op::Push(b), Op::Mul) => {
                    Some(Op::Push((*a).wrapping_mul(*b)))
                }
                (Op::Push(a), Op::Push(b), Op::Div) => {
                    (*a).checked_div(*b).map(Op::Push)
                }
                (Op::Push(a), Op::Push(b), Op::Mod) => {
                    (*a).checked_rem(*b).map(Op::Push)
                }
                _ => None,
            };

            if let Some(op) = folded {
                let new_index = optimized.len();
                optimized.push(op);
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                old_to_new[index + 2] = new_index;
                index += 3;
                continue;
            }
        }

        if index + 1 < program.len() && !targeted[index + 1] {
            let folded = match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => Some(Op::Push((*value).wrapping_neg())),
                _ => None,
            };

            if let Some(op) = folded {
                let new_index = optimized.len();
                optimized.push(op);
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                index += 2;
                continue;
            }
        }

        if index + 1 < program.len()
            && !targeted[index]
            && !targeted[index + 1]
        {
            if matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop)) {
                let new_index = optimized.len();
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                index += 2;
                continue;
            }
        }

        old_to_new[index] = optimized.len();
        optimized.push(copy_op(&program[index]));
        index += 1;
    }

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                if old_target < old_to_new.len() {
                    *target = old_to_new[old_target];
                }
            }
            _ => {}
        }
    }

    optimized
}

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
