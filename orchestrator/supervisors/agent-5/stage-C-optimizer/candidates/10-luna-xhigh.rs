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

fn optimize_region_is_safe(targets: &[bool], start: usize, length: usize) -> bool {
    let mut offset = 1;
    while offset < length {
        if targets[start + offset] {
            return false;
        }
        offset += 1;
    }
    true
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
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

    let mut index_map = vec![0; length + 1];
    let mut optimized = Vec::with_capacity(length);
    let mut index = 0;

    while index < length {
        if length - index >= 3
            && optimize_region_is_safe(&targeted, index, 3)
        {
            if let (Op::Push(a), Op::Push(b)) = (&program[index], &program[index + 1]) {
                let folded = match &program[index + 2] {
                    Op::Add => Some((*a).wrapping_add(*b)),
                    Op::Sub => Some((*a).wrapping_sub(*b)),
                    Op::Mul => Some((*a).wrapping_mul(*b)),
                    Op::Div if *b != 0 => Some((*a).wrapping_div(*b)),
                    Op::Mod if *b != 0 => Some((*a).wrapping_rem(*b)),
                    _ => None,
                };

                if let Some(value) = folded {
                    let output_index = optimized.len();
                    optimized.push(Op::Push(value));
                    index_map[index] = output_index;
                    index_map[index + 1] = output_index;
                    index_map[index + 2] = output_index;
                    index += 3;
                    continue;
                }
            }
        }

        if length - index >= 2
            && optimize_region_is_safe(&targeted, index, 2)
        {
            if let Op::Push(value) = &program[index] {
                if matches!(&program[index + 1], Op::Neg) {
                    let output_index = optimized.len();
                    optimized.push(Op::Push((*value).wrapping_neg()));
                    index_map[index] = output_index;
                    index_map[index + 1] = output_index;
                    index += 2;
                    continue;
                }

                if matches!(&program[index + 1], Op::Pop) {
                    let output_index = optimized.len();
                    index_map[index] = output_index;
                    index_map[index + 1] = output_index;
                    index += 2;
                    continue;
                }
            }
        }

        index_map[index] = optimized.len();
        optimized.push(optimize_copy_op(&program[index]));
        index += 1;
    }

    index_map[length] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= length => {
                let old_target = *target;
                *target = index_map[old_target];
            }
            _ => {}
        }
    }

    optimized
}
