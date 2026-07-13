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

fn ano_copy_program(program: &[Op]) -> Vec<Op> {
    let mut copy = Vec::with_capacity(program.len());
    for op in program {
        copy.push(ano_copy_op(op));
    }
    copy
}

fn ano_wrapping_div(left: i64, right: i64) -> i64 {
    if left == i64::MIN && right == -1 {
        i64::MIN
    } else {
        left / right
    }
}

fn ano_wrapping_mod(left: i64, right: i64) -> i64 {
    if left == i64::MIN && right == -1 {
        0
    } else {
        left % right
    }
}

fn ano_fold_three(first: &Op, second: &Op, third: &Op) -> Option<i64> {
    match (first, second, third) {
        (Op::Push(left), Op::Push(right), Op::Add) => {
            Some((*left).wrapping_add(*right))
        }
        (Op::Push(left), Op::Push(right), Op::Sub) => {
            Some((*left).wrapping_sub(*right))
        }
        (Op::Push(left), Op::Push(right), Op::Mul) => {
            Some((*left).wrapping_mul(*right))
        }
        (Op::Push(left), Op::Push(right), Op::Div) if *right != 0 => {
            Some(ano_wrapping_div(*left, *right))
        }
        (Op::Push(left), Op::Push(right), Op::Mod) if *right != 0 => {
            Some(ano_wrapping_mod(*left, *right))
        }
        _ => None,
    }
}

fn ano_fold_two(first: &Op, second: &Op) -> Option<i64> {
    match (first, second) {
        (Op::Push(value), Op::Neg) => Some((*value).wrapping_neg()),
        _ => None,
    }
}

fn ano_is_push_pop(first: &Op, second: &Op) -> bool {
    match (first, second) {
        (Op::Push(_), Op::Pop) => true,
        _ => false,
    }
}

fn ano_safe_span(start: usize, end: usize, targets: &[bool]) -> bool {
    if start >= end {
        return false;
    }

    let mut index = start + 1;
    while index < end {
        if targets[index] {
            return false;
        }
        index += 1;
    }

    true
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut targets = vec![false; program.len()];
    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < program.len() {
                    targets[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut nodes = Vec::with_capacity(program.len());
    for (index, op) in program.iter().enumerate() {
        nodes.push(AnoOptimizeNode {
            op: ano_copy_op(op),
            start: index,
            end: index + 1,
        });
    }

    let mut changed = true;
    while changed {
        changed = false;
        let mut index = 0;

        while index < nodes.len() {
            if index + 2 < nodes.len() {
                if let Some(value) =
                    ano_fold_three(&nodes[index].op, &nodes[index + 1].op, &nodes[index + 2].op)
                {
                    let start = nodes[index].start;
                    let end = nodes[index + 2].end;

                    if ano_safe_span(start, end, &targets) {
                        nodes[index].op = Op::Push(value);
                        nodes[index].end = end;
                        nodes.drain(index + 1..index + 3);
                        changed = true;
                        if index > 0 {
                            index -= 1;
                        }
                        continue;
                    }
                }
            }

            if index + 1 < nodes.len() {
                if let Some(value) = ano_fold_two(&nodes[index].op, &nodes[index + 1].op) {
                    let start = nodes[index].start;
                    let end = nodes[index + 1].end;

                    if ano_safe_span(start, end, &targets) {
                        nodes[index].op = Op::Push(value);
                        nodes[index].end = end;
                        nodes.drain(index + 1..index + 2);
                        changed = true;
                        if index > 0 {
                            index -= 1;
                        }
                        continue;
                    }
                }

                if ano_is_push_pop(&nodes[index].op, &nodes[index + 1].op) {
                    let start = nodes[index].start;
                    let end = nodes[index + 1].end;

                    if ano_safe_span(start, end, &targets) {
                        nodes.drain(index..index + 2);
                        changed = true;
                        if index > 0 {
                            index -= 1;
                        }
                        continue;
                    }
                }
            }

            index += 1;
        }
    }

    let output_len = nodes.len();
    let mut source_to_output = vec![usize::MAX; program.len()];

    for (output_index, node) in nodes.iter().enumerate() {
        for source_index in node.start..node.end {
            source_to_output[source_index] = output_index;
        }
    }

    let mut next_output = output_len;
    for source_index in (0..program.len()).rev() {
        if source_to_output[source_index] == usize::MAX {
            source_to_output[source_index] = next_output;
        } else {
            next_output = source_to_output[source_index];
        }
    }

    for source_index in 0..program.len() {
        if targets[source_index] && source_to_output[source_index] == output_len {
            return ano_copy_program(program);
        }
    }

    for node in &mut nodes {
        match &mut node.op {
            Op::Jmp(target) | Op::Jz(target) => {
                let old_target = *target;
                if old_target < program.len() {
                    *target = source_to_output[old_target];
                }
            }
            _ => {}
        }
    }

    nodes.into_iter().map(|node| node.op).collect()
}
