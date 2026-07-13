struct PeepholeEntry {
    op: Op,
    start: usize,
    end: usize,
    has_target: bool,
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

fn peephole_copy_program(program: &[Op]) -> Vec<Op> {
    let mut result = Vec::with_capacity(program.len());
    for op in program {
        result.push(peephole_copy_op(op));
    }
    result
}

fn peephole_fold_binary(op: &Op, left: i64, right: i64) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div => {
            if right == 0 {
                None
            } else if left == i64::MIN && right == -1 {
                Some(i64::MIN)
            } else {
                Some(left / right)
            }
        }
        Op::Mod => {
            if right == 0 {
                None
            } else if left == i64::MIN && right == -1 {
                Some(0)
            } else {
                Some(left % right)
            }
        }
        _ => None,
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targets = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return peephole_copy_program(program);
                }
                targets[*target] = true;
            }
            _ => {}
        }
    }

    let mut entries = Vec::with_capacity(length);

    for (index, op) in program.iter().enumerate() {
        entries.push(PeepholeEntry {
            op: peephole_copy_op(op),
            start: index,
            end: index,
            has_target: targets[index],
        });

        loop {
            let entry_count = entries.len();

            if entry_count >= 3
                && !entries[entry_count - 3].has_target
                && !entries[entry_count - 2].has_target
                && !entries[entry_count - 1].has_target
            {
                let folded = {
                    let first = &entries[entry_count - 3];
                    let second = &entries[entry_count - 2];
                    let third = &entries[entry_count - 1];

                    match (&first.op, &second.op) {
                        (Op::Push(left), Op::Push(right)) => {
                            peephole_fold_binary(&third.op, *left, *right)
                        }
                        _ => None,
                    }
                };

                if let Some(value) = folded {
                    let third = entries.pop().unwrap();
                    let second = entries.pop().unwrap();
                    let first = entries.pop().unwrap();

                    entries.push(PeepholeEntry {
                        op: Op::Push(value),
                        start: first.start,
                        end: third.end,
                        has_target: false,
                    });

                    let _ = second;
                    continue;
                }
            }

            let entry_count = entries.len();

            if entry_count >= 2
                && !entries[entry_count - 2].has_target
                && !entries[entry_count - 1].has_target
            {
                let remove_pair = {
                    let first = &entries[entry_count - 2];
                    let second = &entries[entry_count - 1];

                    match (&first.op, &second.op) {
                        (Op::Push(_), Op::Pop) => true,
                        _ => false,
                    }
                };

                if remove_pair {
                    entries.pop();
                    entries.pop();
                    continue;
                }

                let negated = {
                    let first = &entries[entry_count - 2];
                    let second = &entries[entry_count - 1];

                    match (&first.op, &second.op) {
                        (Op::Push(value), Op::Neg) => Some(0i64.wrapping_sub(*value)),
                        _ => None,
                    }
                };

                if let Some(value) = negated {
                    let second = entries.pop().unwrap();
                    let first = entries.pop().unwrap();

                    entries.push(PeepholeEntry {
                        op: Op::Push(value),
                        start: first.start,
                        end: second.end,
                        has_target: false,
                    });

                    continue;
                }
            }

            break;
        }
    }

    let mut mapping = vec![0usize; length];

    for (output_index, entry) in entries.iter().enumerate() {
        let mut source_index = entry.start;
        loop {
            mapping[source_index] = output_index;
            if source_index == entry.end {
                break;
            }
            source_index += 1;
        }
    }

    let mut result = Vec::with_capacity(entries.len());

    for entry in entries {
        let mut op = entry.op;

        match &mut op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                *target = mapping[old_target];
            }
            _ => {}
        }

        result.push(op);
    }

    result
}
