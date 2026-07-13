fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_integer(text: &str) -> Option<i64> {
    let digits = text.strip_prefix('-').unwrap_or(text);

    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }

    text.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !is_label_name(name) {
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

        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        let op = match mnemonic {
            "push" => {
                let operand = tokens.next().ok_or_else(|| "missing operand".to_string())?;
                let value = parse_integer(operand).ok_or_else(|| "invalid operand".to_string())?;
                Op::Push(value)
            }
            "jmp" | "jz" => {
                let operand = tokens.next().ok_or_else(|| "missing operand".to_string())?;
                if !is_label_name(operand) {
                    return Err("invalid operand".into());
                }
                let target = *labels
                    .get(operand)
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
