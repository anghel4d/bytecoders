pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = copy_program(program);

    loop {
        let next = optimize_pass(&current);
        if next.len() == current.len() {
            return next;
        }
        current = next;
    }
}

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target > length {
                    return copy_program(program);
                }
                if *target < length {
                    targeted[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut target_map = vec![0usize; length + 1];
    let mut optimized = Vec::with_capacity(length);
    let mut index = 0;

    while index < length {
        if index + 2 < length {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
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
                if !has_target(&targeted, index, index + 3) {
                    let output_index = optimized.len();
                    target_map[index] = output_index;
                    target_map[index + 1] = output_index;
                    target_map[index + 2] = output_index;
                    optimized.push(Op::Push(value));
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < length {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => {
                    if !has_target(&targeted, index, index + 2) {
                        let output_index = optimized.len();
                        target_map[index] = output_index;
                        target_map[index + 1] = output_index;
                        optimized.push(Op::Push((*value).wrapping_neg()));
                        index += 2;
                        continue;
                    }
                }
                (Op::Push(_), Op::Pop) => {
                    if !has_target(&targeted, index, index + 2) {
                        let output_index = optimized.len();
                        target_map[index] = output_index;
                        target_map[index + 1] = output_index;
                        index += 2;
                        continue;
                    }
                }
                _ => {}
            }
        }

        target_map[index] = optimized.len();
        optimized.push(copy_op(&program[index]));
        index += 1;
    }

    target_map[length] = optimized.len();
    rewrite_jump_targets(&mut optimized, &target_map);
    optimized
}

fn has_target(targeted: &[bool], start: usize, end: usize) -> bool {
    targeted[start..end].iter().any(|target| *target)
}

fn rewrite_jump_targets(program: &mut [Op], target_map: &[usize]) {
    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                *target = target_map[old_target];
            }
            _ => {}
        }
    }
}

fn copy_program(program: &[Op]) -> Vec<Op> {
    let mut copy = Vec::with_capacity(program.len());
    for op in program {
        copy.push(copy_op(op));
    }
    copy
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
