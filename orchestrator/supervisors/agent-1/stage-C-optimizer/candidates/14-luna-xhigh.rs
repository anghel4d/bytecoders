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

fn optimize_remap_target(
    target: usize,
    old_to_new: &[usize],
    mapped: &[bool],
    new_len: usize,
) -> usize {
    if target == old_to_new.len() {
        new_len
    } else if target < old_to_new.len() && mapped[target] {
        old_to_new[target]
    } else {
        target
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let old_len = program.len();
    let mut jump_targets = vec![false; old_len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < old_len => {
                jump_targets[*target] = true;
            }
            _ => {}
        }
    }

    let mut optimized = Vec::with_capacity(old_len);
    let mut old_to_new = vec![0; old_len];
    let mut mapped = vec![false; old_len];
    let mut index = 0;

    while index < old_len {
        if index + 1 < old_len && !jump_targets[index + 1] {
            if matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop)) {
                old_to_new[index] = optimized.len();
                mapped[index] = true;
                index += 2;
                continue;
            }
        }

        if index + 2 < old_len
            && !jump_targets[index + 1]
            && !jump_targets[index + 2]
        {
            let replacement = match (
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
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some(Op::Push((*a).wrapping_div(*b)))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some(Op::Push((*a).wrapping_rem(*b)))
                }
                _ => None,
            };

            if let Some(op) = replacement {
                old_to_new[index] = optimized.len();
                mapped[index] = true;
                optimized.push(op);
                index += 3;
                continue;
            }
        }

        if index + 1 < old_len && !jump_targets[index + 1] {
            if let (Op::Push(value), Op::Neg) = (&program[index], &program[index + 1]) {
                old_to_new[index] = optimized.len();
                mapped[index] = true;
                optimized.push(Op::Push((*value).wrapping_neg()));
                index += 2;
                continue;
            }
        }

        old_to_new[index] = optimized.len();
        mapped[index] = true;
        optimized.push(optimize_clone_op(&program[index]));
        index += 1;
    }

    let new_len = optimized.len();
    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = optimize_remap_target(*target, &old_to_new, &mapped, new_len);
            }
            _ => {}
        }
    }

    optimized
}
