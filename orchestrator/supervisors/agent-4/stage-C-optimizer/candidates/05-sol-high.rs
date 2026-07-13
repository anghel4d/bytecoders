struct OptimizePiece {
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

fn copy_program(program: &[Op]) -> Vec<Op> {
    program.iter().map(copy_op).collect()
}

fn range_has_jump_target(prefix: &[usize], start: usize, end: usize) -> bool {
    start < end && prefix[start] != prefix[end]
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let length = program.len();
    let mut is_jump_target = vec![false; length];

    for op in program {
        match op {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target >= length {
                    return copy_program(program);
                }
                is_jump_target[*target] = true;
            }
            _ => {}
        }
    }

    let mut target_prefix = vec![0usize; length + 1];
    for index in 0..length {
        target_prefix[index + 1] =
            target_prefix[index] + is_jump_target[index] as usize;
    }

    let mut pieces: Vec<OptimizePiece> = Vec::with_capacity(length);

    for (index, op) in program.iter().enumerate() {
        pieces.push(OptimizePiece {
            op: copy_op(op),
            start: index,
            end: index + 1,
        });

        loop {
            let count = pieces.len();

            if count >= 3 {
                let folded = match (
                    &pieces[count - 3].op,
                    &pieces[count - 2].op,
                    &pieces[count - 1].op,
                ) {
                    (Op::Push(a), Op::Push(b), Op::Add) => {
                        Some((*a).wrapping_add(*b))
                    }
                    (Op::Push(a), Op::Push(b), Op::Sub) => {
                        Some((*a).wrapping_sub(*b))
                    }
                    (Op::Push(a), Op::Push(b), Op::Mul) => {
                        Some((*a).wrapping_mul(*b))
                    }
                    (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                        Some((*a).wrapping_div(*b))
                    }
                    (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                        Some((*a).wrapping_rem(*b))
                    }
                    _ => None,
                };

                if let Some(value) = folded {
                    let start = pieces[count - 3].start;
                    let end = pieces[count - 1].end;

                    if !range_has_jump_target(&target_prefix, start + 1, end) {
                        pieces.truncate(count - 3);
                        pieces.push(OptimizePiece {
                            op: Op::Push(value),
                            start,
                            end,
                        });
                        continue;
                    }
                }
            }

            let count = pieces.len();

            if count >= 2 {
                let folded = match (
                    &pieces[count - 2].op,
                    &pieces[count - 1].op,
                ) {
                    (Op::Push(value), Op::Neg) => {
                        Some((*value).wrapping_neg())
                    }
                    _ => None,
                };

                if let Some(value) = folded {
                    let start = pieces[count - 2].start;
                    let end = pieces[count - 1].end;

                    if !range_has_jump_target(&target_prefix, start + 1, end) {
                        pieces.truncate(count - 2);
                        pieces.push(OptimizePiece {
                            op: Op::Push(value),
                            start,
                            end,
                        });
                        continue;
                    }
                }
            }

            let count = pieces.len();

            if count >= 2
                && matches!(pieces[count - 2].op, Op::Push(_))
                && matches!(pieces[count - 1].op, Op::Pop)
            {
                let start = pieces[count - 2].start;
                let end = pieces[count - 1].end;

                if !range_has_jump_target(&target_prefix, start, end) {
                    pieces.truncate(count - 2);
                    continue;
                }
            }

            break;
        }
    }

    let mut remapped_targets = vec![usize::MAX; length];

    for (new_index, piece) in pieces.iter().enumerate() {
        remapped_targets[piece.start] = new_index;
    }

    for old_index in 0..length {
        if is_jump_target[old_index]
            && remapped_targets[old_index] == usize::MAX
        {
            return copy_program(program);
        }
    }

    for piece in &mut pieces {
        match &mut piece.op {
            Op::Jmp(target) | Op::Jz(target) => {
                *target = remapped_targets[*target];
            }
            _ => {}
        }
    }

    pieces.into_iter().map(|piece| piece.op).collect()
}
