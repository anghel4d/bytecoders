fn valid_label(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'_'))
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for raw_line in src.lines() {
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let first = tokens.next().unwrap();

        if let Some(name) = first.strip_suffix(':') {
            if tokens.next().is_some() || !valid_label(name) {
                return Err("invalid label".into());
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".into());
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();
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
                Op::Push(value)
            }
            "jmp" | "jz" => {
                let name = tokens
                    .next()
                    .ok_or_else(|| "missing operand".to_string())?;
                if !valid_label(name) {
                    return Err("invalid operand".into());
                }
                let target = *labels
                    .get(name)
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

        if tokens.next().is_some() {
            return Err("trailing tokens".into());
        }

        ops.push(op);
    }

    Ok(ops)
}
