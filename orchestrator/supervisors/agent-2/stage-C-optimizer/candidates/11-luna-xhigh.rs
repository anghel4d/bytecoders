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

fn remap_target(target: usize, map: &[usize], old_len: usize, new_len: usize) -> usize {
    if target < old_len {
        map[target]
    } else if target == old_len {
        new_len
    } else {
        target
    }
}

fn remap_op(op: &Op, map: &[usize], old_len: usize, new_len: usize) -> Op {
    match op {
        Op::Jmp(target) => Op::Jmp(remap_target(*target, map, old_len, new_len)),
        Op::Jz(target) => Op::Jz(remap_target(*target, map, old_len, new_len)),
        _ => copy_op(op),
    }
}

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let old_len = program.len();
    let mut targeted = vec![false; old_len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < old_len => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut map = vec![0; old_len];
    let mut output = Vec::with_capacity(old_len);
    let mut index = 0;

    while index < old_len {
        if index + 2 < old_len
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
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
                let new_index = output.len();
                map[index] = new_index;
                map[index + 1] = new_index;
                map[index + 2] = new_index;
                output.push(Op::Push(value));
                index += 3;
                continue;
            }
        }

        if index + 1 < old_len && !targeted[index + 1] {
            if let (Op::Push(_), Op::Pop) = (&program[index], &program[index + 1]) {
                let new_index = output.len();
                map[index] = new_index;
                map[index + 1] = new_index;
                index += 2;
                continue;
            }

            if let (Op::Push(value), Op::Neg) = (&program[index], &program[index + 1]) {
                let new_index = output.len();
                map[index] = new_index;
                map[index + 1] = new_index;
                output.push(Op::Push((*value).wrapping_neg()));
                index += 2;
                continue;
            }
        }

        map[index] = output.len();
        output.push(copy_op(&program[index]));
        index += 1;
    }

    let new_len = output.len();
    let mut rewritten = Vec::with_capacity(new_len);

    for op in &output {
        rewritten.push(remap_op(op, &map, old_len, new_len));
    }

    rewritten
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
