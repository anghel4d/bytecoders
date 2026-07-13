pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = ano_opt_copy_program(program);

    loop {
        let (next, changed) = ano_opt_pass(&current);
        if !changed {
            return current;
        }
        current = next;
    }
}

fn ano_opt_copy_program(program: &[Op]) -> Vec<Op> {
    let mut copy = Vec::with_capacity(program.len());

    for op in program {
        copy.push(ano_opt_copy_op(op));
    }

    copy
}

fn ano_opt_copy_op(op: &Op) -> Op {
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

fn ano_opt_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < length => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut consumed = vec![1usize; length];
    let mut replacement: Vec<Option<Op>> =
        (0..length).map(|_| Option::<Op>::None).collect();
    let mut changed = false;
    let mut index = 0;

    while index < length {
        if length - index >= 3
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
                consumed[index] = 3;
                replacement[index] = Some(Op::Push(value));
                changed = true;
                index += 3;
                continue;
            }
        }

        if length - index >= 2 && !targeted[index + 1] {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    consumed[index] = 2;
                    replacement[index] = Some(Op::Push((*value).wrapping_neg()));
                    changed = true;
                    index += 2;
                    continue;
                }
                (Op::Push(_), Op::Pop) => {
                    consumed[index] = 2;
                    changed = true;
                    index += 2;
                    continue;
                }
                _ => {}
            }
        }

        index += 1;
    }

    let mut target_map = vec![0usize; length + 1];
    let mut old_index = 0;
    let mut new_index = 0;

    while old_index < length {
        let width = consumed[old_index];
        let emitted = if replacement[old_index].is_some() || width == 1 {
            1
        } else {
            0
        };

        for offset in 0..width {
            target_map[old_index + offset] = new_index;
        }

        new_index += emitted;
        old_index += width;
    }

    target_map[length] = new_index;

    let mut optimized = Vec::with_capacity(new_index);
    old_index = 0;

    while old_index < length {
        let width = consumed[old_index];

        if let Some(op) = replacement[old_index].as_ref() {
            optimized.push(ano_opt_copy_op(op));
        } else if width == 1 {
            match &program[old_index] {
                Op::Jmp(target) => optimized.push(Op::Jmp(
                    ano_opt_rewrite_target(*target, length, &target_map),
                )),
                Op::Jz(target) => optimized.push(Op::Jz(
                    ano_opt_rewrite_target(*target, length, &target_map),
                )),
                op => optimized.push(ano_opt_copy_op(op)),
            }
        }

        old_index += width;
    }

    (optimized, changed)
}

fn ano_opt_rewrite_target(
    target: usize,
    original_length: usize,
    target_map: &[usize],
) -> usize {
    if target <= original_length {
        target_map[target]
    } else {
        target
    }
}
