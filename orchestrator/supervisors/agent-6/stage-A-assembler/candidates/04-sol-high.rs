fn is_label_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    match bytes.next() {
        Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') => {}
        _ => return false,
    }
    bytes.all(|byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_index = 0;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let first = tokens.next().unwrap();

        if let Some(name) = first.strip_suffix(':') {
            if !is_label_name(name) {
                return Err("invalid label".into());
            }
            if tokens.next().is_some() {
                return Err("trailing tokens".into());
            }
            if labels.insert(name.to_string(), instruction_index).is_some() {
                return Err("duplicate label".into());
            }
        } else {
            instruction_index += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_index);

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        if mnemonic.ends_with(':') {
            continue;
        }

        let op = match mnemonic {
            "push" => {
                let value = tokens
                    .next()
                    .ok_or_else(|| "missing operand".to_string())?
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;
                if tokens.next().is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Push(value)
            }
            "jmp" | "jz" => {
                let label = tokens
                    .next()
                    .ok_or_else(|| "missing operand".to_string())?;
                if !is_label_name(label) {
                    return Err("invalid operand".into());
                }
                if tokens.next().is_some() {
                    return Err("trailing tokens".into());
                }
                let target = *labels
                    .get(label)
                    .ok_or_else(|| "undefined label".to_string())?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
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
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => return Err("unknown mnemonic".into()),
        };

        if !matches!(mnemonic, "push" | "jmp" | "jz") && tokens.next().is_some() {
            return Err("trailing tokens".into());
        }

        ops.push(op);
    }

    Ok(ops)
}
