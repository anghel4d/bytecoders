fn assemble_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn assemble_integer(value: &str) -> bool {
    let bytes = value.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    start < bytes.len() && bytes[start..].iter().all(u8::is_ascii_digit)
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        let text = line.split(';').next().unwrap_or("").trim();

        if text.is_empty() {
            continue;
        }

        let mut tokens = text.split_whitespace();
        let first = tokens.next().unwrap();

        if first.ends_with(':') {
            let name = &first[..first.len() - 1];

            if tokens.next().is_some() {
                return Err("trailing tokens".to_string());
            }
            if !assemble_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name.to_string(), instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count = instruction_count
                .checked_add(1)
                .ok_or_else(|| "too many instructions".to_string())?;
        }
    }

    let mut code = Vec::with_capacity(instruction_count);

    for line in src.lines() {
        let text = line.split(';').next().unwrap_or("").trim();

        if text.is_empty() {
            continue;
        }

        let mut tokens = text.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        if mnemonic.ends_with(':') {
            continue;
        }

        let op = match mnemonic {
            "push" => {
                let value = tokens
                    .next()
                    .ok_or_else(|| "missing operand".to_string())?;

                if !assemble_integer(value) {
                    return Err("invalid operand".to_string());
                }

                Op::Push(
                    value
                        .parse::<i64>()
                        .map_err(|_| "invalid operand".to_string())?,
                )
            }
            "pop" => Op::Pop,
            "add" => Op::Add,
            "sub" => Op::Sub,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "neg" => Op::Neg,
            "dup" => Op::Dup,
            "swap" => Op::Swap,
            "jmp" => {
                let label = tokens
                    .next()
                    .ok_or_else(|| "missing operand".to_string())?;

                Op::Jmp(
                    *labels
                        .get(label)
                        .ok_or_else(|| "undefined label".to_string())?,
                )
            }
            "jz" => {
                let label = tokens
                    .next()
                    .ok_or_else(|| "missing operand".to_string())?;

                Op::Jz(
                    *labels
                        .get(label)
                        .ok_or_else(|| "undefined label".to_string())?,
                )
            }
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => return Err("unknown mnemonic".to_string()),
        };

        if tokens.next().is_some() {
            return Err("trailing tokens".to_string());
        }

        code.push(op);
    }

    Ok(code)
}
