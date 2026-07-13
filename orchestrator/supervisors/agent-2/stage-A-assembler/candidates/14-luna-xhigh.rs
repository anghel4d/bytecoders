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

fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
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

            if !valid_label(name) {
                return Err("invalid label".to_string());
            }

            if labels.insert(name.to_string(), pending.len()).is_some() {
                return Err("duplicate label".to_string());
            }

            continue;
        }

        let op = match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                PendingOp::Push(
                    tokens[1]
                        .parse::<i64>()
                        .map_err(|_| "invalid integer".to_string())?,
                )
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Pop
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Add
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Sub
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Mul
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Div
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Mod
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Neg
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Dup
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Swap
            }
            "jmp" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                if !valid_label(tokens[1]) {
                    return Err("invalid label".to_string());
                }

                PendingOp::Jmp(tokens[1].to_string())
            }
            "jz" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                if !valid_label(tokens[1]) {
                    return Err("invalid label".to_string());
                }

                PendingOp::Jz(tokens[1].to_string())
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Print
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                PendingOp::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        pending.push(op);
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
            PendingOp::Jmp(label) => Op::Jmp(
                *labels
                    .get(&label)
                    .ok_or_else(|| "undefined label".to_string())?,
            ),
            PendingOp::Jz(label) => Op::Jz(
                *labels
                    .get(&label)
                    .ok_or_else(|| "undefined label".to_string())?,
            ),
            PendingOp::Print => Op::Print,
            PendingOp::Halt => Op::Halt,
        };

        output.push(op);
    }

    Ok(output)
}
