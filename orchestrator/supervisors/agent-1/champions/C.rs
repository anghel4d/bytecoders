pub fn optimize(program: &[Op]) -> Vec<Op> {
    let (mut optimized, mut changed) = peephole_optimize_once(program);

    while changed {
        let result = peephole_optimize_once(&optimized);
        optimized = result.0;
        changed = result.1;
    }

    optimized
}

fn peephole_optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
    let len = program.len();
    let mut targeted = vec![false; len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < len => {
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(len);
    let mut old_to_new = vec![0usize; len + 1];
    let mut changed = false;
    let mut i = 0;

    while i < len {
        let new_index = output.len();
        old_to_new[i] = new_index;

        if i + 2 < len && !targeted[i + 1] && !targeted[i + 2] {
            if let Some(folded) =
                peephole_fold_binary(&program[i], &program[i + 1], &program[i + 2])
            {
                old_to_new[i + 1] = new_index;
                old_to_new[i + 2] = new_index;
                output.push(folded);
                changed = true;
                i += 3;
                continue;
            }
        }

        if i + 1 < len && !targeted[i + 1] {
            match (&program[i], &program[i + 1]) {
                (Op::Push(_), Op::Pop) => {
                    old_to_new[i + 1] = new_index;
                    changed = true;
                    i += 2;
                    continue;
                }
                (Op::Push(value), Op::Neg) => {
                    old_to_new[i + 1] = new_index;
                    output.push(Op::Push(value.wrapping_neg()));
                    changed = true;
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        output.push(peephole_copy_op(&program[i]));
        i += 1;
    }

    old_to_new[len] = output.len();

    for op in &mut output {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => target,
            _ => continue,
        };

        if *target <= len {
            *target = old_to_new[*target];
        }
    }

    (output, changed)
}

fn peephole_fold_binary(left: &Op, right: &Op, operator: &Op) -> Option<Op> {
    let (a, b) = match (left, right) {
        (Op::Push(a), Op::Push(b)) => (*a, *b),
        _ => return None,
    };

    let value = match operator {
        Op::Add => a.wrapping_add(b),
        Op::Sub => a.wrapping_sub(b),
        Op::Mul => a.wrapping_mul(b),
        Op::Div if b != 0 => a.wrapping_div(b),
        Op::Mod if b != 0 => a.wrapping_rem(b),
        _ => return None,
    };

    Some(Op::Push(value))
}

fn peephole_copy_op(op: &Op) -> Op {
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
