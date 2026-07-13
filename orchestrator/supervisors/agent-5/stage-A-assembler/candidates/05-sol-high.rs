fn valid_label(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'_'))
        && bytes.all(|byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}

fn parse_integer(text: &str) -> Result<i64, String> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("invalid integer".to_string());
    }
    text.parse().map_err(|_| "invalid integer".to_string())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for line in src.lines() {
        let line = line.split_once(';').map_or(line, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }

        let mut words = line.split_whitespace();
        let first = words.next().unwrap();

        if let Some(name) = first.strip_suffix(':') {
            if words.next().is_some() {
                return Err("trailing tokens".to_string());
            }
            if !valid_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for line in src.lines() {
        let line = line.split_once(';').map_or(line, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }

        let mut words = line.split_whitespace();
        let mnemonic = words.next().unwrap();

        if mnemonic.ends_with(':') {
            continue;
        }

        let op = match mnemonic {
            "push" => {
                let operand = words.next().ok_or_else(|| "missing operand".to_string())?;
                if words.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                Op::Push(parse_integer(operand)?)
            }
            "jmp" | "jz" => {
                let name = words.next().ok_or_else(|| "missing operand".to_string())?;
                if words.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                if !valid_label(name) {
                    return Err("invalid operand".to_string());
                }
                let target = labels
                    .get(name)
                    .copied()
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
            _ => return Err("unknown mnemonic".to_string()),
        };

        if words.next().is_some() {
            return Err("trailing tokens".to_string());
        }
        ops.push(op);
    }

    Ok(ops)
}
