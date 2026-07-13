fn folded_div(a: i64, b: i64) -> i64 {
    if a == i64::MIN && b == -1 {
        i64::MIN
    } else {
        a / b
    }
}

fn folded_mod(a: i64, b: i64) -> i64 {
    if a == i64::MIN && b == -1 {
        0
    } else {
        a % b
    }
}

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let n = program.len();
    let mut targeted = vec![false; n];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < n {
                    targeted[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut staged = Vec::with_capacity(n);
    let mut old_to_new = vec![0usize; n + 1];
    let mut i = 0;

    while i < n {
        if i + 2 < n && !targeted[i + 1] && !targeted[i + 2] {
            let folded = match (&program[i], &program[i + 1], &program[i + 2]) {
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
                    Some(Op::Push(folded_div(*a, *b)))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some(Op::Push(folded_mod(*a, *b)))
                }
                _ => None,
            };

            if let Some(op) = folded {
                let new_index = staged.len();
                staged.push(op);
                old_to_new[i] = new_index;
                old_to_new[i + 1] = new_index;
                old_to_new[i + 2] = new_index;
                i += 3;
                continue;
            }
        }

        if i + 1 < n && !targeted[i + 1] {
            let folded = match (&program[i], &program[i + 1]) {
                (Op::Push(value), Op::Neg) => {
                    Some(Op::Push((*value).wrapping_neg()))
                }
                _ => None,
            };

            if let Some(op) = folded {
                let new_index = staged.len();
                staged.push(op);
                old_to_new[i] = new_index;
                old_to_new[i + 1] = new_index;
                i += 2;
                continue;
            }
        }

        if i + 1 < n && !targeted[i] && !targeted[i + 1] {
            let is_push_pop = match (&program[i], &program[i + 1]) {
                (Op::Push(_), Op::Pop) => true,
                _ => false,
            };

            if is_push_pop {
                let new_index = staged.len();
                old_to_new[i] = new_index;
                old_to_new[i + 1] = new_index;
                i += 2;
                continue;
            }
        }

        old_to_new[i] = staged.len();
        staged.push(match &program[i] {
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
        });
        i += 1;
    }

    old_to_new[n] = staged.len();

    staged
        .into_iter()
        .map(|op| match op {
            Op::Jmp(target) => {
                Op::Jmp(if target < n { old_to_new[target] } else { target })
            }
            Op::Jz(target) => {
                Op::Jz(if target < n { old_to_new[target] } else { target })
            }
            Op::Push(value) => Op::Push(value),
            Op::Pop => Op::Pop,
            Op::Add => Op::Add,
            Op::Sub => Op::Sub,
            Op::Mul => Op::Mul,
            Op::Div => Op::Div,
            Op::Mod => Op::Mod,
            Op::Neg => Op::Neg,
            Op::Dup => Op::Dup,
            Op::Swap => Op::Swap,
            Op::Print => Op::Print,
            Op::Halt => Op::Halt,
        })
        .collect()
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = optimize_pass(program);

    loop {
        let next = optimize_pass(&current);
        if next.len() == current.len() {
            return current;
        }
        current = next;
    }
}
