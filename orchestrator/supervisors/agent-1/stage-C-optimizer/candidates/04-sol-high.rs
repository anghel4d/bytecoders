pub fn optimize(program: &[Op]) -> Vec<Op> {
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

    fn fold_binary(left: i64, right: i64, op: &Op) -> Option<i64> {
        match op {
            Op::Add => Some(left.wrapping_add(right)),
            Op::Sub => Some(left.wrapping_sub(right)),
            Op::Mul => Some(left.wrapping_mul(right)),
            Op::Div if right != 0 => Some(left.wrapping_div(right)),
            Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
            _ => None,
        }
    }

    fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
        let len = program.len();
        let mut targeted = vec![false; len];

        for op in program {
            let target = match op {
                Op::Jmp(target) | Op::Jz(target) => *target,
                _ => continue,
            };

            if target > len {
                return (program.iter().map(copy_op).collect(), false);
            }
            if target < len {
                targeted[target] = true;
            }
        }

        let mut remap = vec![0; len];
        let mut optimized = Vec::with_capacity(len);
        let mut changed = false;
        let mut i = 0;

        while i < len {
            if len - i >= 3 && !targeted[i + 1] && !targeted[i + 2] {
                if let (Op::Push(left), Op::Push(right)) =
                    (&program[i], &program[i + 1])
                {
                    if let Some(value) = fold_binary(*left, *right, &program[i + 2]) {
                        let new_index = optimized.len();
                        remap[i] = new_index;
                        remap[i + 1] = new_index;
                        remap[i + 2] = new_index;
                        optimized.push(Op::Push(value));
                        changed = true;
                        i += 3;
                        continue;
                    }
                }
            }

            if len - i >= 2 && !targeted[i + 1] {
                if let (Op::Push(value), Op::Neg) = (&program[i], &program[i + 1]) {
                    let new_index = optimized.len();
                    remap[i] = new_index;
                    remap[i + 1] = new_index;
                    optimized.push(Op::Push(value.wrapping_neg()));
                    changed = true;
                    i += 2;
                    continue;
                }

                if let (Op::Push(_), Op::Pop) = (&program[i], &program[i + 1]) {
                    let new_index = optimized.len();
                    remap[i] = new_index;
                    remap[i + 1] = new_index;
                    changed = true;
                    i += 2;
                    continue;
                }
            }

            remap[i] = optimized.len();
            optimized.push(copy_op(&program[i]));
            i += 1;
        }

        let new_len = optimized.len();
        for op in &mut optimized {
            match op {
                Op::Jmp(target) | Op::Jz(target) => {
                    *target = if *target == len {
                        new_len
                    } else {
                        remap[*target]
                    };
                }
                _ => {}
            }
        }

        (optimized, changed)
    }

    let (mut current, mut changed) = optimize_pass(program);
    while changed {
        let (next, next_changed) = optimize_pass(&current);
        current = next;
        changed = next_changed;
    }
    current
}
