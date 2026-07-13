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

    fn fold_binary(a: i64, b: i64, op: &Op) -> Option<i64> {
        match op {
            Op::Add => Some(a.wrapping_add(b)),
            Op::Sub => Some(a.wrapping_sub(b)),
            Op::Mul => Some(a.wrapping_mul(b)),
            Op::Div if b != 0 => Some(a.wrapping_div(b)),
            Op::Mod if b != 0 => Some(a.wrapping_rem(b)),
            _ => None,
        }
    }

    fn optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
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
        let mut old_to_new = vec![0; len + 1];
        let mut changed = false;
        let mut i = 0;

        while i < len {
            old_to_new[i] = output.len();

            if i + 2 < len && !targeted[i + 1] && !targeted[i + 2] {
                if let (Op::Push(a), Op::Push(b)) = (&program[i], &program[i + 1]) {
                    if let Some(value) = fold_binary(*a, *b, &program[i + 2]) {
                        old_to_new[i + 1] = output.len();
                        old_to_new[i + 2] = output.len();
                        output.push(Op::Push(value));
                        i += 3;
                        changed = true;
                        continue;
                    }
                }
            }

            if i + 1 < len && !targeted[i + 1] {
                match (&program[i], &program[i + 1]) {
                    (Op::Push(value), Op::Neg) => {
                        old_to_new[i + 1] = output.len();
                        output.push(Op::Push(value.wrapping_neg()));
                        i += 2;
                        changed = true;
                        continue;
                    }
                    (Op::Push(_), Op::Pop) if !targeted[i] => {
                        old_to_new[i + 1] = output.len();
                        i += 2;
                        changed = true;
                        continue;
                    }
                    _ => {}
                }
            }

            output.push(copy_op(&program[i]));
            i += 1;
        }

        old_to_new[len] = output.len();

        for op in &mut output {
            match op {
                Op::Jmp(target) | Op::Jz(target) if *target <= len => {
                    *target = old_to_new[*target];
                }
                _ => {}
            }
        }

        (output, changed)
    }

    let mut current: Vec<Op> = program.iter().map(copy_op).collect();

    loop {
        let (next, changed) = optimize_once(&current);
        if !changed {
            return next;
        }
        current = next;
    }
}
