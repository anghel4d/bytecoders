struct OptimizeNode {
    op: Op,
    start: usize,
    end: usize,
}

fn optimize_clone_op(op: &Op) -> Op {
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

fn optimize_no_jump_target(targets: &[bool], start: usize, end: usize) -> bool {
    let mut index = start;
    while index < end {
        if targets[index] {
            return false;
        }
        index += 1;
    }
    true
}

fn optimize_unary_value(value: &Op, operation: &Op) -> Option<i64> {
    match (value, operation) {
        (Op::Push(value), Op::Neg) => Some(value.wrapping_neg()),
        _ => None,
    }
}

fn optimize_binary_value(left: &Op, right: &Op, operation: &Op) -> Option<i64> {
    let left = match left {
        Op::Push(value) => *value,
        _ => return None,
    };
    let right = match right {
        Op::Push(value) => *value,
        _ => return None,
    };

    match operation {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div if right != 0 => Some(left.wrapping_div(right)),
        Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
        _ => None,
    }
}

fn optimize_reduce(nodes: &mut Vec<OptimizeNode>, targets: &[bool]) {
    loop {
        let length = nodes.len();

        if length >= 2 {
            let first = length - 2;
            let second = length - 1;

            if matches!((&nodes[first].op, &nodes[second].op), (Op::Push(_), Op::Pop))
                && optimize_no_jump_target(targets, nodes[first].start, nodes[second].end)
            {
                nodes.truncate(first);
                continue;
            }

            if let Some(value) =
                optimize_unary_value(&nodes[first].op, &nodes[second].op)
            {
                let start = nodes[first].start;
                let end = nodes[second].end;

                if optimize_no_jump_target(targets, start + 1, end) {
                    nodes.truncate(first);
                    nodes.push(OptimizeNode {
                        op: Op::Push(value),
                        start,
                        end,
                    });
                    continue;
                }
            }
        }

        if length >= 3 {
            let first = length - 3;
            let second = length - 2;
            let third = length - 1;

            if let Some(value) = optimize_binary_value(
                &nodes[first].op,
                &nodes[second].op,
                &nodes[third].op,
            ) {
                let start = nodes[first].start;
                let end = nodes[third].end;

                if optimize_no_jump_target(targets, start + 1, end) {
                    nodes.truncate(first);
                    nodes.push(OptimizeNode {
                        op: Op::Push(value),
                        start,
                        end,
                    });
                    continue;
                }
            }
        }

        break;
    }
}

fn optimize_target(
    target: usize,
    old_len: usize,
    old_to_new: &[Option<usize>],
    new_len: usize,
) -> usize {
    if target < old_len {
        match old_to_new[target] {
            Some(mapped) => mapped,
            None => target,
        }
    } else if target == old_len {
        new_len
    } else {
        target
    }
}

fn optimize_remap_op(
    op: &Op,
    old_len: usize,
    old_to_new: &[Option<usize>],
    new_len: usize,
) -> Op {
    match op {
        Op::Jmp(target) => Op::Jmp(optimize_target(
            *target,
            old_len,
            old_to_new,
            new_len,
        )),
        Op::Jz(target) => Op::Jz(optimize_target(
            *target,
            old_len,
            old_to_new,
            new_len,
        )),
        _ => optimize_clone_op(op),
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let old_len = program.len();
    let mut targets = vec![false; old_len];

    let mut index = 0;
    while index < old_len {
        match &program[index] {
            Op::Jmp(target) | Op::Jz(target) if *target < old_len => {
                targets[*target] = true;
            }
            _ => {}
        }
        index += 1;
    }

    let mut nodes = Vec::with_capacity(old_len);
    for (index, op) in program.iter().enumerate() {
        nodes.push(OptimizeNode {
            op: optimize_clone_op(op),
            start: index,
            end: index + 1,
        });
        optimize_reduce(&mut nodes, &targets);
    }

    let new_len = nodes.len();
    let mut old_to_new = vec![None; old_len];

    for (index, node) in nodes.iter().enumerate() {
        old_to_new[node.start] = Some(index);
    }

    let mut optimized = Vec::with_capacity(new_len);
    for node in &nodes {
        optimized.push(optimize_remap_op(
            &node.op,
            old_len,
            &old_to_new,
            new_len,
        ));
    }

    optimized
}
