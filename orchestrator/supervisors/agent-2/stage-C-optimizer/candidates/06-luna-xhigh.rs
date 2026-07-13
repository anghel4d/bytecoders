pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = program.iter().map(optimizer_copy_op).collect::<Vec<_>>();

    loop {
        let next = optimizer_pass(&current);
        if next.len() == current.len() {
            return next;
        }
        current = next;
    }
}

fn optimizer_copy_op(op: &Op) -> Op {
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

fn optimizer_pass(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targets = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < length {
                    targets[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut group_lengths = vec![1usize; length];
    let mut index = 0;

    while index < length {
        let mut group_length = 1;

        if length - index >= 3 && !targets[index + 1] && !targets[index + 2] {
            let foldable = match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(_), Op::Push(_), Op::Add)
                | (Op::Push(_), Op::Push(_), Op::Sub)
                | (Op::Push(_), Op::Push(_), Op::Mul) => true,
                (Op::Push(_), Op::Push(divisor), Op::Div)
                | (Op::Push(_), Op::Push(divisor), Op::Mod) => *divisor != 0,
                _ => false,
            };

            if foldable {
                group_length = 3;
            }
        }

        if group_length == 1 && length - index >= 2 && !targets[index + 1] {
            let foldable = match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) | (Op::Push(_), Op::Neg) => true,
                _ => false,
            };

            if foldable {
                group_length = 2;
            }
        }

        group_lengths[index] = group_length;
        index += group_length;
    }

    let mut mapping = vec![0usize; length + 1];
    let mut optimized_length = 0;
    index = 0;

    while index < length {
        let group_length = group_lengths[index];
        mapping[index] = optimized_length;

        let mut interior = index + 1;
        while interior < index + group_length {
            mapping[interior] = optimized_length;
            interior += 1;
        }

        optimized_length += 1;
        index += group_length;
    }

    mapping[length] = optimized_length;

    let removed = length - optimized_length;
    let mut optimized = Vec::with_capacity(optimized_length);
    index = 0;

    while index < length {
        match group_lengths[index] {
            1 => optimized.push(optimizer_remap_op(
                &program[index],
                &mapping,
                removed,
            )),
            2 => match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Pop) => {
                    let _ = value;
                }
                (Op::Push(value), Op::Neg) => {
                    optimized.push(Op::Push((*value).wrapping_neg()));
                }
                _ => unreachable!(),
            },
            3 => match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(left), Op::Push(right), Op::Add) => {
                    optimized.push(Op::Push((*left).wrapping_add(*right)));
                }
                (Op::Push(left), Op::Push(right), Op::Sub) => {
                    optimized.push(Op::Push((*left).wrapping_sub(*right)));
                }
                (Op::Push(left), Op::Push(right), Op::Mul) => {
                    optimized.push(Op::Push((*left).wrapping_mul(*right)));
                }
                (Op::Push(left), Op::Push(right), Op::Div) if *right != 0 => {
                    optimized.push(Op::Push((*left).wrapping_div(*right)));
                }
                (Op::Push(left), Op::Push(right), Op::Mod) if *right != 0 => {
                    optimized.push(Op::Push((*left).wrapping_rem(*right)));
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }

        index += group_lengths[index];
    }

    optimized
}

fn optimizer_remap_op(op: &Op, mapping: &[usize], removed: usize) -> Op {
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
        Op::Jmp(target) => Op::Jmp(optimizer_remap_target(*target, mapping, removed)),
        Op::Jz(target) => Op::Jz(optimizer_remap_target(*target, mapping, removed)),
        Op::Print => Op::Print,
        Op::Halt => Op::Halt,
    }
}

fn optimizer_remap_target(target: usize, mapping: &[usize], removed: usize) -> usize {
    let original_length = mapping.len() - 1;

    if target <= original_length {
        mapping[target]
    } else {
        target - removed
    }
}
