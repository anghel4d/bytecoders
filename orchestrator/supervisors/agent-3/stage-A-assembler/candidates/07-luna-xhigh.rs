enum PendingOp {
    Push(i64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Dup,
    Swap,
    Jmp(String),
    Jz(String),
    Print,
    Halt,
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut pending = Vec::new();

    for line in src.lines() {
        let line = line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }

            let name = &tokens[0][..tokens[0].len() - 1];
            if !is_label_name(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name.to_string(), pending.len()).is_some() {
                return Err("duplicate label".to_string());
            }
            continue;
        }

        pending.push(parse_pending(&tokens)?);
    }

    let mut output = Vec::with_capacity(pending.len());

    for op in pending {
        let op = match op {
            PendingOp::Push(value) => Op::Push(value),
            PendingOp::Pop => Op::Pop,
            PendingOp::Add => Op::Add,
            PendingOp::Sub => Op::Sub,
            PendingOp::Mul => Op::Mul,
            PendingOp::Div => Op::Div,
            PendingOp::Mod => Op::Mod,
            PendingOp::Neg => Op::Neg,
            PendingOp::Dup => Op::Dup,
            PendingOp::Swap => Op::Swap,
            PendingOp::Jmp(label) => {
                let target = labels
                    .get(&label)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jmp(target)
            }
            PendingOp::Jz(label) => {
                let target = labels
                    .get(&label)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jz(target)
            }
            PendingOp::Print => Op::Print,
            PendingOp::Halt => Op::Halt,
        };
        output.push(op);
    }

    Ok(output)
}

fn parse_pending(tokens: &[&str]) -> Result<PendingOp, String> {
    match tokens[0] {
        "push" => {
            let operand = one_operand(tokens)?;
            let value = operand
                .parse::<i64>()
                .map_err(|_| "invalid integer".to_string())?;
            Ok(PendingOp::Push(value))
        }
        "jmp" => {
            let operand = one_operand(tokens)?;
            if !is_label_name(operand) {
                return Err("invalid label".to_string());
            }
            Ok(PendingOp::Jmp(operand.to_string()))
        }
        "jz" => {
            let operand = one_operand(tokens)?;
            if !is_label_name(operand) {
                return Err("invalid label".to_string());
            }
            Ok(PendingOp::Jz(operand.to_string()))
        }
        "pop" => {
            no_operands(tokens)?;
            Ok(PendingOp::Pop)
        }
        "add" => {
            no_operands(tokens)?;
            Ok(PendingOp::Add)
        }
        "sub" => {
            no_operands(tokens)?;
            Ok(PendingOp::Sub)
        }
        "mul" => {
            no_operands(tokens)?;
            Ok(PendingOp::Mul)
        }
        "div" => {
            no_operands(tokens)?;
            Ok(PendingOp::Div)
        }
        "mod" => {
            no_operands(tokens)?;
            Ok(PendingOp::Mod)
        }
        "neg" => {
            no_operands(tokens)?;
            Ok(PendingOp::Neg)
        }
        "dup" => {
            no_operands(tokens)?;
            Ok(PendingOp::Dup)
        }
        "swap" => {
            no_operands(tokens)?;
            Ok(PendingOp::Swap)
        }
        "print" => {
            no_operands(tokens)?;
            Ok(PendingOp::Print)
        }
        "halt" => {
            no_operands(tokens)?;
            Ok(PendingOp::Halt)
        }
        _ => Err("unknown mnemonic".to_string()),
    }
}

fn one_operand(tokens: &[&str]) -> Result<&str, String> {
    match tokens.len() {
        1 => Err("missing operand".to_string()),
        2 => Ok(tokens[1]),
        _ => Err("trailing tokens".to_string()),
    }
}

fn no_operands(tokens: &[&str]) -> Result<(), String> {
    if tokens.len() == 1 {
        Ok(())
    } else {
        Err("trailing tokens".to_string())
    }
}

fn is_label_name(name: &str) -> bool {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(b'_') | Some(b'a'..=b'z') | Some(b'A'..=b'Z') => {}
        _ => return false,
    }

    bytes.all(|byte| {
        matches!(
            byte,
            b'_' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9'
        )
    })
}
