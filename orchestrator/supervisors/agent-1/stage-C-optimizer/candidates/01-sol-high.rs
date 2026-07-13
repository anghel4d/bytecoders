pub fn optimize(program: &[Op]) -> Vec<Op> {
    struct Node {
        start: usize,
        end: usize,
        op: Op,
    }

    fn clone_op(op: &Op) -> Op {
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

    fn clone_program(program: &[Op]) -> Vec<Op> {
        program.iter().map(clone_op).collect()
    }

    let len = program.len();
    let mut targeted = vec![false; len];

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => *target,
            _ => continue,
        };

        if target > len {
            return clone_program(program);
        }
        if target < len {
            targeted[target] = true;
        }
    }

    let mut target_prefix = vec![0usize; len + 1];
    for index in 0..len {
        target_prefix[index + 1] = target_prefix[index] + usize::from(targeted[index]);
    }

    let has_target = |start: usize, end: usize| {
        target_prefix[end] != target_prefix[start]
    };

    let mut nodes: Vec<Node> = program
        .iter()
        .enumerate()
        .map(|(index, op)| Node {
            start: index,
            end: index + 1,
            op: clone_op(op),
        })
        .collect();

    loop {
        let mut next = Vec::with_capacity(nodes.len());
        let mut index = 0;
        let mut changed = false;

        while index < nodes.len() {
            if index + 2 < nodes.len()
                && nodes[index].end == nodes[index + 1].start
                && nodes[index + 1].end == nodes[index + 2].start
                && !has_target(nodes[index].start, nodes[index + 2].end)
            {
                if let (Op::Push(a), Op::Push(b)) =
                    (&nodes[index].op, &nodes[index + 1].op)
                {
                    let value = match &nodes[index + 2].op {
                        Op::Add => Some((*a).wrapping_add(*b)),
                        Op::Sub => Some((*a).wrapping_sub(*b)),
                        Op::Mul => Some((*a).wrapping_mul(*b)),
                        Op::Div if *b != 0 => Some((*a).wrapping_div(*b)),
                        Op::Mod if *b != 0 => Some((*a).wrapping_rem(*b)),
                        _ => None,
                    };

                    if let Some(value) = value {
                        next.push(Node {
                            start: nodes[index].start,
                            end: nodes[index + 2].end,
                            op: Op::Push(value),
                        });
                        index += 3;
                        changed = true;
                        continue;
                    }
                }
            }

            if index + 1 < nodes.len()
                && nodes[index].end == nodes[index + 1].start
                && !has_target(nodes[index].start, nodes[index + 1].end)
            {
                match (&nodes[index].op, &nodes[index + 1].op) {
                    (Op::Push(value), Op::Neg) => {
                        next.push(Node {
                            start: nodes[index].start,
                            end: nodes[index + 1].end,
                            op: Op::Push(value.wrapping_neg()),
                        });
                        index += 2;
                        changed = true;
                        continue;
                    }
                    (Op::Push(_), Op::Pop) => {
                        index += 2;
                        changed = true;
                        continue;
                    }
                    _ => {}
                }
            }

            next.push(Node {
                start: nodes[index].start,
                end: nodes[index].end,
                op: clone_op(&nodes[index].op),
            });
            index += 1;
        }

        nodes = next;
        if !changed {
            break;
        }
    }

    let mut old_to_new = vec![None; len + 1];
    for (new_index, node) in nodes.iter().enumerate() {
        old_to_new[node.start] = Some(new_index);
    }
    old_to_new[len] = Some(nodes.len());

    for op in program {
        if let Op::Jmp(target) | Op::Jz(target) = op {
            if old_to_new[*target].is_none() {
                return clone_program(program);
            }
        }
    }

    nodes
        .into_iter()
        .map(|node| match node.op {
            Op::Jmp(target) => Op::Jmp(old_to_new[target].unwrap()),
            Op::Jz(target) => Op::Jz(old_to_new[target].unwrap()),
            op => op,
        })
        .collect()
}
