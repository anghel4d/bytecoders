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

    fn copy_program(program: &[Op]) -> Vec<Op> {
        program.iter().map(copy_op).collect()
    }

    fn optimize_once(program: &[Op]) -> (Vec<Op>, bool) {
        let mut targeted = vec![false; program.len()];

        for op in program {
            match op {
                Op::Jmp(target) | Op::Jz(target) if *target < program.len() => {
                    targeted[*target] = true;
                }
                _ => {}
            }
        }

        let mut output = Vec::with_capacity(program.len());
        let mut old_to_new = vec![0; program.len() + 1];
        let mut changed = false;
        let mut i = 0;

        while i < program.len() {
            let mut rewrite = None;

            if i + 2 < program.len() {
                rewrite = match (&program[i], &program[i + 1], &program[i + 2]) {
                    (Op::Push(a), Op::Push(b), Op::Add) => {
                        Some((3, Some(Op::Push((*a).wrapping_add(*b)))))
                    }
                    (Op::Push(a), Op::Push(b), Op::Sub) => {
                        Some((3, Some(Op::Push((*a).wrapping_sub(*b)))))
                    }
                    (Op::Push(a), Op::Push(b), Op::Mul) => {
                        Some((3, Some(Op::Push((*a).wrapping_mul(*b)))))
                    }
                    (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                        Some((3, Some(Op::Push((*a).wrapping_div(*b)))))
                    }
                    (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                        Some((3, Some(Op::Push((*a).wrapping_rem(*b)))))
                    }
                    _ => None,
                };
            }

            if rewrite.is_none() && i + 1 < program.len() {
                rewrite = match (&program[i], &program[i + 1]) {
                    (Op::Push(value), Op::Neg) => {
                        Some((2, Some(Op::Push((*value).wrapping_neg()))))
                    }
                    (Op::Push(_), Op::Pop) => Some((2, None)),
                    _ => None,
                };
            }

            if let Some((width, replacement)) = rewrite {
                if (i + 1..i + width).all(|index| !targeted[index]) {
                    let new_index = output.len();

                    for old_index in i..i + width {
                        old_to_new[old_index] = new_index;
                    }

                    if let Some(op) = replacement {
                        output.push(op);
                    }

                    i += width;
                    changed = true;
                    continue;
                }
            }

            old_to_new[i] = output.len();
            output.push(copy_op(&program[i]));
            i += 1;
        }

        old_to_new[program.len()] = output.len();

        for op in &mut output {
            match op {
                Op::Jmp(target) | Op::Jz(target) => {
                    *target = old_to_new[*target];
                }
                _ => {}
            }
        }

        (output, changed)
    }

    if program.iter().any(|op| {
        matches!(
            op,
            Op::Jmp(target) | Op::Jz(target) if *target > program.len()
        )
    }) {
        return copy_program(program);
    }

    let mut optimized = copy_program(program);

    loop {
        let (next, changed) = optimize_once(&optimized);
        optimized = next;

        if !changed {
            return optimized;
        }
    }
}
