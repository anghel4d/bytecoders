#[derive(Clone, Copy)]
enum Peephole {
    Keep,
    Fold { value: i64, width: usize },
    Remove { width: usize },
    Skip,
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let mut current = optimize_pass(program);

    loop {
        let next = optimize_pass(&current);
        if next.len() >= current.len() {
            return current;
        }
        current = next;
    }
}

fn optimize_pass(program: &[Op]) -> Vec<Op> {
    let mut targeted = vec![false; program.len()];

    for instruction in program {
        match instruction {
            Op::Jmp(target) | Op::Jz(target) => {
                if *target < targeted.len() {
                    targeted[*target] = true;
                }
            }
            _ => {}
        }
    }

    let mut rewrites = Vec::with_capacity(program.len());
    rewrites.resize(program.len(), Peephole::Keep);

    let mut index = 0;
    while index < program.len() {
        if let (Some(&Op::Push(_)), Some(&Op::Pop)) =
            (program.get(index), program.get(index + 1))
        {
            if !targeted[index + 1] {
                rewrites[index] = Peephole::Remove { width: 2 };
                rewrites[index + 1] = Peephole::Skip;
                index += 2;
                continue;
            }
        }

        if let Some((value, width)) = fold_at(program, index) {
            let mut safe = true;
            for offset in 1..width {
                if targeted[index + offset] {
                    safe = false;
                    break;
                }
            }

            if safe {
                rewrites[index] = Peephole::Fold { value, width };
                for offset in 1..width {
                    rewrites[index + offset] = Peephole::Skip;
                }
                index += width;
                continue;
            }
        }

        index += 1;
    }

    let mut mapping = vec![0; program.len() + 1];
    let mut new_index = 0;
    index = 0;

    while index < program.len() {
        mapping[index] = new_index;

        match rewrites[index] {
            Peephole::Keep => {
                new_index += 1;
                index += 1;
            }
            Peephole::Fold { width, .. } | Peephole::Remove { width } => {
                for offset in 1..width {
                    mapping[index + offset] = new_index;
                }
                new_index += 1;
                index += width;
            }
            Peephole::Skip => unreachable!(),
        }
    }

    mapping[program.len()] = new_index;

    let mut optimized = Vec::with_capacity(new_index);
    index = 0;

    while index < program.len() {
        match rewrites[index] {
            Peephole::Keep => {
                optimized.push(rewrite_op(&program[index], &mapping));
                index += 1;
            }
            Peephole::Fold { value, width } => {
                optimized.push(Op::Push(value));
                index += width;
            }
            Peephole::Remove { width } => {
                index += width;
            }
            Peephole::Skip => unreachable!(),
        }
    }

    optimized
}

fn fold_at(program: &[Op], index: usize) -> Option<(i64, usize)> {
    match (program.get(index), program.get(index + 1)) {
        (Some(&Op::Push(value)), Some(&Op::Neg)) => {
            Some((value.wrapping_neg(), 2))
        }
        (Some(&Op::Push(left)), Some(&Op::Push(right))) => {
            match program.get(index + 2) {
                Some(&Op::Add) => Some((left.wrapping_add(right), 3)),
                Some(&Op::Sub) => Some((left.wrapping_sub(right), 3)),
                Some(&Op::Mul) => Some((left.wrapping_mul(right), 3)),
                Some(&Op::Div) if right != 0 => {
                    Some((fold_div(left, right), 3))
                }
                Some(&Op::Mod) if right != 0 => {
                    Some((fold_mod(left, right), 3))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn fold_div(left: i64, right: i64) -> i64 {
    if left == i64::MIN && right == -1 {
        i64::MIN
    } else {
        left / right
    }
}

fn fold_mod(left: i64, right: i64) -> i64 {
    if left == i64::MIN && right == -1 {
        0
    } else {
        left % right
    }
}

fn rewrite_op(instruction: &Op, mapping: &[usize]) -> Op {
    match instruction {
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
        Op::Jmp(target) => Op::Jmp(remap_target(*target, mapping)),
        Op::Jz(target) => Op::Jz(remap_target(*target, mapping)),
        Op::Print => Op::Print,
        Op::Halt => Op::Halt,
    }
}

fn remap_target(target: usize, mapping: &[usize]) -> usize {
    if target < mapping.len() - 1 {
        mapping[target]
    } else {
        target
    }
}
