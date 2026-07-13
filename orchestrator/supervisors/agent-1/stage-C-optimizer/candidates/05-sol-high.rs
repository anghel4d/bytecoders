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

    let mut current: Vec<Op> = program.iter().map(copy_op).collect();

    loop {
        let len = current.len();
        let mut targeted = vec![false; len + 1];

        for op in &current {
            match op {
                Op::Jmp(target) | Op::Jz(target) => {
                    if *target > len {
                        return current;
                    }
                    targeted[*target] = true;
                }
                _ => {}
            }
        }

        let mut result = Vec::with_capacity(len);
        let mut old_to_new = vec![0; len + 1];
        let mut changed = false;
        let mut i = 0;

        while i < len {
            if i + 2 < len && !targeted[i + 1] && !targeted[i + 2] {
                let folded = match (&current[i], &current[i + 1], &current[i + 2]) {
                    (Op::Push(a), Op::Push(b), Op::Add) => Some(a.wrapping_add(*b)),
                    (Op::Push(a), Op::Push(b), Op::Sub) => Some(a.wrapping_sub(*b)),
                    (Op::Push(a), Op::Push(b), Op::Mul) => Some(a.wrapping_mul(*b)),
                    (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                        Some(a.wrapping_div(*b))
                    }
                    (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                        Some(a.wrapping_rem(*b))
                    }
                    _ => None,
                };

                if let Some(value) = folded {
                    let new_index = result.len();
                    old_to_new[i] = new_index;
                    old_to_new[i + 1] = new_index;
                    old_to_new[i + 2] = new_index;
                    result.push(Op::Push(value));
                    i += 3;
                    changed = true;
                    continue;
                }
            }

            if i + 1 < len && !targeted[i + 1] {
                match (&current[i], &current[i + 1]) {
                    (Op::Push(value), Op::Neg) => {
                        let new_index = result.len();
                        old_to_new[i] = new_index;
                        old_to_new[i + 1] = new_index;
                        result.push(Op::Push(value.wrapping_neg()));
                        i += 2;
                        changed = true;
                        continue;
                    }
                    (Op::Push(_), Op::Pop) => {
                        let new_index = result.len();
                        old_to_new[i] = new_index;
                        old_to_new[i + 1] = new_index;
                        i += 2;
                        changed = true;
                        continue;
                    }
                    _ => {}
                }
            }

            old_to_new[i] = result.len();
            result.push(copy_op(&current[i]));
            i += 1;
        }

        old_to_new[len] = result.len();

        if !changed {
            return current;
        }

        for op in &mut result {
            match op {
                Op::Jmp(target) | Op::Jz(target) => {
                    *target = old_to_new[*target];
                }
                _ => {}
            }
        }

        current = result;
    }
}
