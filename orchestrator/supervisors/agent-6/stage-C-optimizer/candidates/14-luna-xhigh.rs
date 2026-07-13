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

fn optimize_once(program: &[Op]) -> Vec<Op> {
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

    let mut old_to_new = vec![0; old_len];
    let mut result = Vec::with_capacity(old_len);
    let mut index = 0;

    while index < old_len {
        let new_index = result.len();

        if old_len - index >= 3 && !targeted[index + 1] && !targeted[index + 2] {
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
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                old_to_new[index + 2] = new_index;
                result.push(Op::Push(value));
                index += 3;
                continue;
            }
        }

        if old_len - index >= 2 {
            match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) if !targeted[index + 1] => {
                    old_to_new[index] = new_index;
                    old_to_new[index + 1] = new_index;
                    result.push(Op::Push((*value).wrapping_neg()));
                    index += 2;
                    continue;
                }
                (Op::Push(_), Op::Pop) if !targeted[index] && !targeted[index + 1] => {
                    old_to_new[index] = new_index;
                    old_to_new[index + 1] = new_index;
                    index += 2;
                    continue;
                }
                _ => {}
            }
        }

        old_to_new[index] = new_index;
        result.push(optimize_clone_op(&program[index]));
        index += 1;
    }

    let new_len = result.len();
    for op in result.iter_mut() {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < old_len {
                    *target = old_to_new[*target];
                } else if *target == old_len {
                    *target = new_len;
                }
            }
            _ => {}
        }
    }

    result
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = program.iter().map(optimize_clone_op).collect::<Vec<_>>();

    loop {
        let next = optimize_once(&current);
        if next.len() == current.len() {
            return next;
        }
        current = next;
    }
}
