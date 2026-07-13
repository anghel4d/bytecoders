fn ano_opt_copy_op(op: &Op) -> Op {
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

fn ano_opt_unreferenced(targets: &[bool], start: usize, length: usize) -> bool {
    targets[start..start + length].iter().all(|targeted| !*targeted)
}

fn ano_opt_pass(program: &[Op]) -> Vec<Op> {
    let mut targeted = vec![false; program.len()];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => Some(*target),
            _ => None,
        };

        if let Some(target) = target {
            if target < program.len() {
                targeted[target] = true;
            }
        }
    }

    let mut new_index = vec![0usize; program.len()];
    let mut retained = vec![false; program.len()];
    let mut optimized = Vec::with_capacity(program.len());
    let mut index = 0;

    while index < program.len() {
        if index + 2 < program.len() && ano_opt_unreferenced(&targeted, index, 3) {
            let replacement = match (&program[index], &program[index + 1], &program[index + 2]) {
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

            if let Some(op) = replacement {
                new_index[index] = optimized.len();
                retained[index] = true;
                optimized.push(op);
                index += 3;
                continue;
            }
        }

        if index + 1 < program.len() && ano_opt_unreferenced(&targeted, index, 2) {
            match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) => {
                    index += 2;
                    continue;
                }
                (Op::Push(value), Op::Neg) => {
                    new_index[index] = optimized.len();
                    retained[index] = true;
                    optimized.push(Op::Push((*value).wrapping_neg()));
                    index += 2;
                    continue;
                }
                _ => {}
            }
        }

        new_index[index] = optimized.len();
        retained[index] = true;
        optimized.push(ano_opt_copy_op(&program[index]));
        index += 1;
    }

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < program.len() && retained[*target] {
                    *target = new_index[*target];
                }
            }
            _ => {}
        }
    }

    optimized
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = ano_opt_pass(program);

    loop {
        let next = ano_opt_pass(&current);
        if next.len() >= current.len() {
            return current;
        }
        current = next;
    }
}
