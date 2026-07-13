pub fn optimize(program: &[Op]) -> Vec<Op> {
    ano_optimize_pass(program)
}

struct AnoOptimizeNode {
    op: Op,
    start: usize,
    end: usize,
}

fn ano_copy_op(op: &Op) -> Op {
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

fn ano_fold_binary(left: i64, right: i64, op: &Op) -> Option<i64> {
    match op {
        Op::Add => Some(left.wrapping_add(right)),
        Op::Sub => Some(left.wrapping_sub(right)),
        Op::Mul => Some(left.wrapping_mul(right)),
        Op::Div if right != 0 => Some(left.wrapping_div(right)),
        Op::Mod if right != 0 => Some(left.wrapping_rem(right)),
        _ => None,
    }
}

fn ano_optimize_pass(program: &[Op]) -> Vec<Op> {
    let mut jump_targets = vec![false; program.len()];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < program.len() {
                    jump_targets[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut target_prefix = vec![0usize; program.len() + 1];
    for index in 0..program.len() {
        target_prefix[index + 1] =
            target_prefix[index] + if jump_targets[index] { 1 } else { 0 };
    }

    let has_target = |start: usize, end: usize| {
        target_prefix[end] != target_prefix[start]
    };

    let mut nodes = Vec::with_capacity(program.len());

    for index in 0..program.len() {
        nodes.push(AnoOptimizeNode {
            op: ano_copy_op(&program[index]),
            start: index,
            end: index + 1,
        });

        loop {
            let length = nodes.len();

            if length >= 2 {
                let remove_push_pop = match (&nodes[length - 2].op, &nodes[length - 1].op) {
                    (Op::Push(_), Op::Pop) => {
                        !has_target(nodes[length - 2].start, nodes[length - 1].end)
                    }
                    _ => false,
                };

                if remove_push_pop {
                    nodes.truncate(length - 2);
                    continue;
                }

                let folded_neg = match (&nodes[length - 2].op, &nodes[length - 1].op) {
                    (Op::Push(value), Op::Neg)
                        if !has_target(nodes[length - 2].start, nodes[length - 1].end) =>
                    {
                        Some((*value, nodes[length - 2].start, nodes[length - 1].end))
                    }
                    _ => None,
                };

                if let Some((value, start, end)) = folded_neg {
                    nodes.truncate(length - 2);
                    nodes.push(AnoOptimizeNode {
                        op: Op::Push(value.wrapping_neg()),
                        start,
                        end,
                    });
                    continue;
                }
            }

            if length >= 3 {
                let folded_binary = match (
                    &nodes[length - 3].op,
                    &nodes[length - 2].op,
                    &nodes[length - 1].op,
                ) {
                    (Op::Push(left), Op::Push(right), operation)
                        if !has_target(nodes[length - 3].start, nodes[length - 1].end) =>
                    {
                        ano_fold_binary(*left, *right, operation).map(|value| {
                            (value, nodes[length - 3].start, nodes[length - 1].end)
                        })
                    }
                    _ => None,
                };

                if let Some((value, start, end)) = folded_binary {
                    nodes.truncate(length - 3);
                    nodes.push(AnoOptimizeNode {
                        op: Op::Push(value),
                        start,
                        end,
                    });
                    continue;
                }
            }

            break;
        }
    }

    let mut old_to_new = vec![0usize; program.len() + 1];
    let mut optimized = Vec::with_capacity(nodes.len());

    for (output_index, node) in nodes.into_iter().enumerate() {
        let AnoOptimizeNode { op, start, end } = node;

        for original_index in start..end {
            old_to_new[original_index] = output_index;
        }

        optimized.push(op);
    }

    old_to_new[program.len()] = optimized.len();

    for op in &mut optimized {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target <= program.len() {
                    *target = old_to_new[*target];
                }
            }
            _ => {}
        }
    }

    optimized
}
