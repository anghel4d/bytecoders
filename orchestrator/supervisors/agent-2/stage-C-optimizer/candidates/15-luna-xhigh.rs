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

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut targeted = vec![false; program.len()];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < program.len() {
                    targeted[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut old_to_new = vec![0usize; program.len()];
    let mut output = Vec::with_capacity(program.len());
    let mut index = 0;

    while index < program.len() {
        if index + 2 < program.len()
            && !targeted[index]
            && !targeted[index + 1]
            && !targeted[index + 2]
        {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
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

            if let Some(op) = folded {
                let new_index = output.len();
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                old_to_new[index + 2] = new_index;
                output.push(op);
                index += 3;
                continue;
            }
        }

        if index + 1 < program.len() && !targeted[index] && !targeted[index + 1] {
            if let (Op::Push(value), Op::Neg) = (&program[index], &program[index + 1]) {
                let new_index = output.len();
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                output.push(Op::Push((*value).wrapping_neg()));
                index += 2;
                continue;
            }

            if matches!((&program[index], &program[index + 1]), (Op::Push(_), Op::Pop)) {
                let new_index = output.len();
                old_to_new[index] = new_index;
                old_to_new[index + 1] = new_index;
                index += 2;
                continue;
            }
        }

        old_to_new[index] = output.len();
        output.push(copy_op(&program[index]));
        index += 1;
    }

    let mut result = Vec::with_capacity(output.len());

    for op in output {
        match op {
            Op::Jmp(target) => {
                result.push(Op::Jmp(if target < old_to_new.len() {
                    old_to_new[target]
                } else {
                    target
                }));
            }
            Op::Jz(target) => {
                result.push(Op::Jz(if target < old_to_new.len() {
                    old_to_new[target]
                } else {
                    target
                }));
            }
            op => result.push(op),
        }
    }

    result
}
