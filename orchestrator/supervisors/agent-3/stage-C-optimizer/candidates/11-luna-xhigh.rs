pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = Vec::with_capacity(program.len());
    for op in program {
        current.push(clone_op(op));
    }

    loop {
        let next = optimize_pass(&current);
        if next.len() >= current.len() {
            return current;
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
                if *target < length {
                    targeted[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut old_to_new = vec![0; length];
    let mut index = 0;

    while index < length {
        if length - index >= 2 {
            let removable = match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) => !targeted[index] && !targeted[index + 1],
                _ => false,
            };

            if removable {
                let output_index = output.len();
                old_to_new[index] = output_index;
                old_to_new[index + 1] = output_index;
                index += 2;
                continue;
            }

            if !targeted[index + 1] {
                let folded = match (&program[index], &program[index + 1]) {
                    (Op::Push(value), Op::Neg) => Some((*value).wrapping_neg()),
                    _ => None,
                };

                if let Some(value) = folded {
                    let output_index = output.len();
                    old_to_new[index] = output_index;
                    old_to_new[index + 1] = output_index;
                    output.push(Op::Push(value));
                    index += 2;
                    continue;
                }
            }
        }

        if length - index >= 3 && !targeted[index + 1] && !targeted[index + 2] {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(a), Op::Push(b), op) => fold_binary(op, *a, *b),
                _ => None,
            };

            if let Some(value) = folded {
                let output_index = output.len();
                old_to_new[index] = output_index;
                old_to_new[index + 1] = output_index;
                old_to_new[index + 2] = output_index;
                output.push(Op::Push(value));
                index += 3;
                continue;
            }
        }

        old_to_new[index] = output.len();
        output.push(clone_op(&program[index]));
        index += 1;
    }

    for op in &mut output {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < length {
                    *target = old_to_new[*target];
                }
            }
            _ => {}
        }
    }

    output
}

fn fold_binary(op: &Op, left: i64, right: i64) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div if right != 0 => Some(left.wrapping_div(right)),
        Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
        _ => None,
    }
}

fn clone_op(op: &Op) -> Op {
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
