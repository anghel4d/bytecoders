struct OptimizeEntry {
    op: Op,
    start: usize,
    end: usize,
}

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

fn optimize_has_target(targets: &[bool], start: usize, end: usize) -> bool {
    targets[start..end].iter().any(|target| *target)
}

fn optimize_remap_target(
    target: usize,
    old_len: usize,
    mapping: &[usize],
    new_len: usize,
) -> usize {
    if target < old_len {
        mapping[target]
    } else if target == old_len {
        new_len
    } else {
        target
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let old_len = program.len();
    let mut jump_targets = vec![false; old_len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < old_len => {
                jump_targets[*target] = true;
            }
            _ => {}
        }
    }

    let mut optimized = Vec::with_capacity(old_len);
    let mut index = 0;

    while index < old_len {
        if index + 2 < old_len {
            let folded = match (&program[index], &program[index + 1], &program[index + 2]) {
                (Op::Push(a), Op::Push(b), Op::Add) => Some((*a).wrapping_add(*b)),
                (Op::Push(a), Op::Push(b), Op::Sub) => Some((*a).wrapping_sub(*b)),
                (Op::Push(a), Op::Push(b), Op::Mul) => Some((*a).wrapping_mul(*b)),
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some((*a).wrapping_div(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some((*a).wrapping_rem(*b))
                }
                _ => None,
            };

            if let Some(value) = folded {
                if !optimize_has_target(&jump_targets, index + 1, index + 3) {
                    optimized.push(OptimizeEntry {
                        op: Op::Push(value),
                        start: index,
                        end: index + 3,
                    });
                    index += 3;
                    continue;
                }
            }
        }

        if index + 1 < old_len {
            let folded = match (&program[index], &program[index + 1]) {
                (Op::Push(value), Op::Neg) => Some((*value).wrapping_neg()),
                _ => None,
            };

            if let Some(value) = folded {
                if !optimize_has_target(&jump_targets, index + 1, index + 2) {
                    optimized.push(OptimizeEntry {
                        op: Op::Push(value),
                        start: index,
                        end: index + 2,
                    });
                    index += 2;
                    continue;
                }
            }
        }

        if index + 1 < old_len {
            let is_push_pop = match (&program[index], &program[index + 1]) {
                (Op::Push(_), Op::Pop) => true,
                _ => false,
            };

            if is_push_pop && !optimize_has_target(&jump_targets, index, index + 2) {
                index += 2;
                continue;
            }
        }

        optimized.push(OptimizeEntry {
            op: optimize_copy_op(&program[index]),
            start: index,
            end: index + 1,
        });
        index += 1;
    }

    let new_len = optimized.len();
    let mut mapping = vec![0; old_len + 1];

    for (new_index, entry) in optimized.iter().enumerate() {
        for old_index in entry.start..entry.end {
            mapping[old_index] = new_index;
        }
    }
    mapping[old_len] = new_len;

    let mut result = Vec::with_capacity(new_len);

    for entry in optimized {
        match entry.op {
            Op::Jmp(target) => result.push(Op::Jmp(optimize_remap_target(
                target, old_len, &mapping, new_len,
            ))),
            Op::Jz(target) => result.push(Op::Jz(optimize_remap_target(
                target, old_len, &mapping, new_len,
            ))),
            op => result.push(op),
        }
    }

    result
}
