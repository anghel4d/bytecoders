pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = optimizer_optimize_once(program);

    loop {
        let next = optimizer_optimize_once(&current);
        if next.len() >= current.len() {
            return current;
        }
        current = next;
    }
}

fn optimizer_optimize_once(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = Vec::with_capacity(length);

    for _ in 0..length {
        targeted.push(false);
    }

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return optimizer_clone_program(program);
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut planned: Vec<Option<Op>> = Vec::with_capacity(length);
    let mut index = 0;

    while index < length {
        if index + 2 < length && !targeted[index + 1] && !targeted[index + 2] {
            if let (&Op::Push(left), &Op::Push(right)) =
                (&program[index], &program[index + 1])
            {
                if let Some(folded) =
                    optimizer_fold_binary(&program[index + 2], left, right)
                {
                    planned.push(Some(folded));
                    planned.push(None);
                    planned.push(None);
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < length && !targeted[index + 1] {
            if let (&Op::Push(value), &Op::Neg) =
                (&program[index], &program[index + 1])
            {
                planned.push(Some(Op::Push(value.wrapping_neg())));
                planned.push(None);
                index += 2;
                continue;
            }
        }

        if index + 1 < length
            && !targeted[index + 1]
            && (!targeted[index] || index + 2 < length)
        {
            if let (&Op::Push(_), &Op::Pop) =
                (&program[index], &program[index + 1])
            {
                planned.push(None);
                planned.push(None);
                index += 2;
                continue;
            }
        }

        planned.push(Some(optimizer_clone_op(&program[index])));
        index += 1;
    }

    let mut source_to_output = Vec::with_capacity(length + 1);
    let mut output_index = 0;

    for instruction in &planned {
        source_to_output.push(output_index);
        if instruction.is_some() {
            output_index += 1;
        }
    }
    source_to_output.push(output_index);

    let mut optimized = Vec::with_capacity(output_index);

    for instruction in planned {
        if let Some(op) = instruction {
            let rewritten = match op {
                Op::Jmp(target) => Op::Jmp(optimizer_remap_target(
                    target,
                    length,
                    &source_to_output,
                )),
                Op::Jz(target) => Op::Jz(optimizer_remap_target(
                    target,
                    length,
                    &source_to_output,
                )),
                other => other,
            };
            optimized.push(rewritten);
        }
    }

    optimized
}

fn optimizer_fold_binary(op: &Op, left: i64, right: i64) -> Option<Op> {
    let value = match op {
        Op::Add => left.wrapping_add(right),
        Op::Sub => left.wrapping_sub(right),
        Op::Mul => left.wrapping_mul(right),
        Op::Div => {
            if right == 0 {
                return None;
            }
            if left == i64::MIN && right == -1 {
                i64::MIN
            } else {
                left / right
            }
        }
        Op::Mod => {
            if right == 0 {
                return None;
            }
            if left == i64::MIN && right == -1 {
                0
            } else {
                left % right
            }
        }
        _ => return None,
    };

    Some(Op::Push(value))
}

fn optimizer_remap_target(
    target: usize,
    original_length: usize,
    source_to_output: &[usize],
) -> usize {
    if target <= original_length {
        source_to_output[target]
    } else {
        target
    }
}

fn optimizer_clone_program(program: &[Op]) -> Vec<Op> {
    let mut cloned = Vec::with_capacity(program.len());

    for op in program {
        cloned.push(optimizer_clone_op(op));
    }

    cloned
}

fn optimizer_clone_op(op: &Op) -> Op {
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
