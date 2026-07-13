pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut targets = vec![false; program.len() + 1];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) if *target <= program.len() => {
                targets[*target] = true;
            }
            _ => {}
        }
    }

    let mut chunks = Vec::with_capacity(program.len());

    for (index, op) in program.iter().enumerate() {
        chunks.push(PeepholeChunk {
            op: copy_op(op),
            start: index,
            end: index + 1,
        });

        while reduce_tail(&mut chunks, &targets) {}
    }

    let mut old_to_new = vec![0; program.len() + 1];
    let mut chunk_index = 0;

    for old_index in 0..=program.len() {
        while chunk_index < chunks.len() && chunks[chunk_index].end <= old_index {
            chunk_index += 1;
        }
        old_to_new[old_index] = chunk_index;
    }

    chunks
        .into_iter()
        .map(|chunk| remap_jumps(chunk.op, &old_to_new, program.len()))
        .collect()
}

struct PeepholeChunk {
    op: Op,
    start: usize,
    end: usize,
}

fn reduce_tail(chunks: &mut Vec<PeepholeChunk>, targets: &[bool]) -> bool {
    let len = chunks.len();

    if len >= 3
        && !targets[chunks[len - 2].start]
        && !targets[chunks[len - 1].start]
    {
        let folded = match (
            &chunks[len - 3].op,
            &chunks[len - 2].op,
            &chunks[len - 1].op,
        ) {
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
            let start = chunks[len - 3].start;
            let end = chunks[len - 1].end;
            chunks.truncate(len - 3);
            chunks.push(PeepholeChunk {
                op: Op::Push(value),
                start,
                end,
            });
            return true;
        }
    }

    if len >= 2 && !targets[chunks[len - 1].start] {
        let negated = match (&chunks[len - 2].op, &chunks[len - 1].op) {
            (Op::Push(value), Op::Neg) => Some((*value).wrapping_neg()),
            _ => None,
        };

        if let Some(value) = negated {
            let start = chunks[len - 2].start;
            let end = chunks[len - 1].end;
            chunks.truncate(len - 2);
            chunks.push(PeepholeChunk {
                op: Op::Push(value),
                start,
                end,
            });
            return true;
        }

        if matches!(
            (&chunks[len - 2].op, &chunks[len - 1].op),
            (Op::Push(_), Op::Pop)
        ) {
            chunks.truncate(len - 2);
            return true;
        }
    }

    false
}

fn remap_jumps(op: Op, old_to_new: &[usize], old_len: usize) -> Op {
    match op {
        Op::Jmp(target) if target <= old_len => Op::Jmp(old_to_new[target]),
        Op::Jz(target) if target <= old_len => Op::Jz(old_to_new[target]),
        other => other,
    }
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
