pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut targeted = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return program.iter().map(peephole_clone_op).collect();
                }
                targeted[*target] = true;
            }
            _ => {}
        }
    }

    let mut target_prefix = vec![0usize; length + 1];
    for index in 0..length {
        target_prefix[index + 1] =
            target_prefix[index] + if targeted[index] { 1 } else { 0 };
    }

    let mut chunks = Vec::with_capacity(length);

    for (index, op) in program.iter().enumerate() {
        chunks.push(PeepholeChunk {
            op: peephole_clone_op(op),
            start: index,
            end: index + 1,
        });

        loop {
            let chunk_count = chunks.len();

            if chunk_count >= 3 {
                let first = chunk_count - 3;
                let start = chunks[first].start;
                let end = chunks[chunk_count - 1].end;

                if peephole_region_is_safe(&target_prefix, start, end) {
                    if let Some(value) = peephole_fold_three(
                        &chunks[first].op,
                        &chunks[first + 1].op,
                        &chunks[first + 2].op,
                    ) {
                        chunks.truncate(first);
                        chunks.push(PeepholeChunk {
                            op: Op::Push(value),
                            start,
                            end,
                        });
                        continue;
                    }
                }
            }

            let chunk_count = chunks.len();

            if chunk_count >= 2 {
                let first = chunk_count - 2;
                let start = chunks[first].start;
                let end = chunks[chunk_count - 1].end;

                if peephole_region_is_safe(&target_prefix, start, end) {
                    if let Some(value) =
                        peephole_fold_two(&chunks[first].op, &chunks[first + 1].op)
                    {
                        chunks.truncate(first);
                        chunks.push(PeepholeChunk {
                            op: Op::Push(value),
                            start,
                            end,
                        });
                        continue;
                    }

                    if peephole_is_push_pop(&chunks[first].op, &chunks[first + 1].op) {
                        chunks.truncate(first);
                        continue;
                    }
                }
            }

            break;
        }
    }

    let mut target_map = vec![0usize; length + 1];
    let mut chunk_index = 0;

    for original_index in 0..=length {
        while chunk_index < chunks.len() && chunks[chunk_index].end <= original_index {
            chunk_index += 1;
        }
        target_map[original_index] = chunk_index;
    }

    let mut result = Vec::with_capacity(chunks.len());

    for chunk in chunks {
        match chunk.op {
            Op::Jmp(target) => result.push(Op::Jmp(target_map[target])),
            Op::Jz(target) => result.push(Op::Jz(target_map[target])),
            op => result.push(op),
        }
    }

    result
}

struct PeepholeChunk {
    op: Op,
    start: usize,
    end: usize,
}

fn peephole_clone_op(op: &Op) -> Op {
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

fn peephole_region_is_safe(prefix: &[usize], start: usize, end: usize) -> bool {
    prefix[end] == prefix[start + 1]
}

fn peephole_fold_three(first: &Op, second: &Op, third: &Op) -> Option<i64> {
    match (first, second, third) {
        (Op::Push(a), Op::Push(b), Op::Add) => Some((*a).wrapping_add(*b)),
        (Op::Push(a), Op::Push(b), Op::Sub) => Some((*a).wrapping_sub(*b)),
        (Op::Push(a), Op::Push(b), Op::Mul) => Some((*a).wrapping_mul(*b)),
        (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => Some((*a).wrapping_div(*b)),
        (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => Some((*a).wrapping_rem(*b)),
        _ => None,
    }
}

fn peephole_fold_two(first: &Op, second: &Op) -> Option<i64> {
    match (first, second) {
        (Op::Push(value), Op::Neg) => Some((*value).wrapping_neg()),
        _ => None,
    }
}

fn peephole_is_push_pop(first: &Op, second: &Op) -> bool {
    match (first, second) {
        (Op::Push(_), Op::Pop) => true,
        _ => false,
    }
}
