struct OptimizeItem {
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

fn optimize_push_value(op: &Op) -> Option<i64> {
    match op {
        Op::Push(value) => Some(*value),
        _ => None,
    }
}

fn optimize_binary(a: i64, b: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(a.wrapping_add(b)),
        Op::Sub => Some(a.wrapping_sub(b)),
        Op::Mul => Some(a.wrapping_mul(b)),
        Op::Div if b != 0 => Some(a.wrapping_div(b)),
        Op::Mod if b != 0 => Some(a.wrapping_rem(b)),
        _ => None,
    }
}

fn optimize_safe_span(targets: &[bool], start: usize, end: usize) -> bool {
    targets[start + 1..end].iter().all(|targeted| !*targeted)
}

fn optimize_target(target: usize, old_len: usize, remap: &[usize]) -> usize {
    if target <= old_len {
        remap[target]
    } else {
        target
    }
}

fn optimize_retarget_op(op: Op, old_len: usize, remap: &[usize]) -> Op {
    match op {
        Op::Jmp(target) => Op::Jmp(optimize_target(target, old_len, remap)),
        Op::Jz(target) => Op::Jz(optimize_target(target, old_len, remap)),
        op => op,
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let old_len = program.len();
    let mut targets = vec![false; old_len];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target < old_len => {
                targets[*target] = true;
            }
            _ => {}
        }
    }

    let mut items = Vec::with_capacity(old_len);

    for (index, op) in program.iter().enumerate() {
        items.push(OptimizeItem {
            op: optimize_copy_op(op),
            start: index,
            end: index + 1,
        });

        loop {
            let len = items.len();

            if len >= 2 {
                let left = len - 2;
                let right = len - 1;

                if items[left].end == items[right].start
                    && optimize_safe_span(&targets, items[left].start, items[right].end)
                    && optimize_push_value(&items[left].op).is_some()
                    && matches!(&items[right].op, Op::Pop)
                {
                    items.truncate(left);
                    continue;
                }

                if items[left].end == items[right].start
                    && optimize_safe_span(&targets, items[left].start, items[right].end)
                    && matches!(&items[right].op, Op::Neg)
                {
                    if let Some(value) = optimize_push_value(&items[left].op) {
                        let end = items[right].end;
                        items[left].op = Op::Push(value.wrapping_neg());
                        items[left].end = end;
                        items.pop();
                        continue;
                    }
                }
            }

            let len = items.len();
            if len >= 3 {
                let first = len - 3;
                let second = len - 2;
                let third = len - 1;

                if items[first].end == items[second].start
                    && items[second].end == items[third].start
                    && optimize_safe_span(&targets, items[first].start, items[third].end)
                {
                    let result = match (
                        optimize_push_value(&items[first].op),
                        optimize_push_value(&items[second].op),
                    ) {
                        (Some(a), Some(b)) => optimize_binary(a, b, &items[third].op),
                        _ => None,
                    };

                    if let Some(value) = result {
                        let end = items[third].end;
                        items[first].op = Op::Push(value);
                        items[first].end = end;
                        items.truncate(first + 1);
                        continue;
                    }
                }
            }

            break;
        }
    }

    let mut remap = vec![0usize; old_len + 1];
    let mut old_index = 0;
    let mut new_index = 0;

    for item in &items {
        while old_index < item.start {
            remap[old_index] = new_index;
            old_index += 1;
        }

        for index in item.start..item.end {
            remap[index] = new_index;
        }

        old_index = item.end;
        new_index += 1;
    }

    while old_index < old_len {
        remap[old_index] = new_index;
        old_index += 1;
    }

    remap[old_len] = new_index;

    let mut optimized = Vec::with_capacity(items.len());
    for item in items {
        optimized.push(optimize_retarget_op(item.op, old_len, &remap));
    }

    optimized
}
