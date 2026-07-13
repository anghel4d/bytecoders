pub fn optimize(program: &[Op]) -> Vec<Op> {
    struct Item {
        op: Op,
        start: usize,
        end: usize,
    }

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

    fn contains_target(targets: &[bool], start: usize, end: usize) -> bool {
        targets[start..end].iter().any(|target| *target)
    }

    let mut targets = vec![false; program.len()];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => Some(*target),
            _ => None,
        };

        if let Some(target) = target {
            if target >= program.len() {
                return program.iter().map(copy_op).collect();
            }
            targets[target] = true;
        }
    }

    let mut items: Vec<Item> = Vec::with_capacity(program.len());

    for (index, op) in program.iter().enumerate() {
        items.push(Item {
            op: copy_op(op),
            start: index,
            end: index + 1,
        });

        loop {
            let len = items.len();

            let folded = if len >= 3 {
                let first = &items[len - 3];
                let second = &items[len - 2];
                let third = &items[len - 1];

                if first.end == second.start
                    && second.end == third.start
                    && !contains_target(&targets, first.start + 1, third.end)
                {
                    let value = match (&first.op, &second.op, &third.op) {
                        (Op::Push(a), Op::Push(b), Op::Add) => {
                            Some(a.wrapping_add(*b))
                        }
                        (Op::Push(a), Op::Push(b), Op::Sub) => {
                            Some(a.wrapping_sub(*b))
                        }
                        (Op::Push(a), Op::Push(b), Op::Mul) => {
                            Some(a.wrapping_mul(*b))
                        }
                        (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                            Some(a.wrapping_div(*b))
                        }
                        (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                            Some(a.wrapping_rem(*b))
                        }
                        _ => None,
                    };

                    value.map(|value| (first.start, third.end, Op::Push(value)))
                } else {
                    None
                }
            } else {
                None
            };

            if let Some((start, end, op)) = folded {
                items.truncate(len - 3);
                items.push(Item { op, start, end });
                continue;
            }

            let negated = if len >= 2 {
                let first = &items[len - 2];
                let second = &items[len - 1];

                if first.end == second.start
                    && !contains_target(&targets, first.start + 1, second.end)
                {
                    match (&first.op, &second.op) {
                        (Op::Push(value), Op::Neg) => {
                            Some((first.start, second.end, Op::Push(value.wrapping_neg())))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if let Some((start, end, op)) = negated {
                items.truncate(len - 2);
                items.push(Item { op, start, end });
                continue;
            }

            let removable = if len >= 2 {
                let first = &items[len - 2];
                let second = &items[len - 1];

                first.end == second.start
                    && matches!((&first.op, &second.op), (Op::Push(_), Op::Pop))
                    && !contains_target(&targets, first.start, second.end)
            } else {
                false
            };

            if removable {
                items.truncate(len - 2);
                continue;
            }

            break;
        }
    }

    let mut old_to_new = vec![usize::MAX; program.len()];

    for (new_index, item) in items.iter().enumerate() {
        old_to_new[item.start] = new_index;
    }

    for (old_index, targeted) in targets.iter().enumerate() {
        if *targeted && old_to_new[old_index] == usize::MAX {
            return program.iter().map(copy_op).collect();
        }
    }

    items
        .into_iter()
        .map(|item| match item.op {
            Op::Jmp(target) => Op::Jmp(old_to_new[target]),
            Op::Jz(target) => Op::Jz(old_to_new[target]),
            op => op,
        })
        .collect()
}
