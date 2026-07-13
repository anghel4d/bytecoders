fn optimize_copy_op(op: &Op) -> Op {
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

fn optimize_peephole(program: &[Op], start: usize) -> Option<(usize, Option<Op>)> {
    let remaining = program.len() - start;

    if remaining >= 3 {
        let folded = match (&program[start], &program[start + 1], &program[start + 2]) {
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

        if folded.is_some() {
            return folded;
        }
    }

    if remaining >= 2 {
        return match (&program[start], &program[start + 1]) {
            (Op::Push(_), Op::Pop) => Some((2, None)),
            (Op::Push(value), Op::Neg) => {
                Some((2, Some(Op::Push((*value).wrapping_neg()))))
            }
            _ => None,
        };
    }

    None
}

fn optimize_has_incoming(incoming: &[bool], start: usize, length: usize) -> bool {
    incoming[start..start + length]
        .iter()
        .any(|targeted| *targeted)
}

fn optimize_pass(program: &[Op]) -> (Vec<Op>, bool) {
    let length = program.len();
    let mut incoming = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) if *target < length => incoming[*target] = true,
            Op::Jz(target) if *target < length => incoming[*target] = true,
            _ => {}
        }
    }

    let mut output = Vec::with_capacity(length);
    let mut source_to_output = vec![0usize; length];
    let mut changed = false;
    let mut index = 0;

    while index < length {
        if let Some((span, replacement)) = optimize_peephole(program, index) {
            if !optimize_has_incoming(&incoming, index, span) {
                let output_index = output.len();

                for source_index in index..index + span {
                    source_to_output[source_index] = output_index;
                }

                if let Some(op) = replacement {
                    output.push(op);
                }

                index += span;
                changed = true;
                continue;
            }
        }

        source_to_output[index] = output.len();
        output.push(optimize_copy_op(&program[index]));
        index += 1;
    }

    for op in &mut output {
        match op {
            Op::Jmp(target) if *target < length => {
                *target = source_to_output[*target];
            }
            Op::Jz(target) if *target < length => {
                *target = source_to_output[*target];
            }
            _ => {}
        }
    }

    (output, changed)
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = Vec::with_capacity(program.len());

    for op in program {
        current.push(optimize_copy_op(op));
    }

    loop {
        let (next, changed) = optimize_pass(&current);

        if !changed {
            return next;
        }

        current = next;
    }
}
