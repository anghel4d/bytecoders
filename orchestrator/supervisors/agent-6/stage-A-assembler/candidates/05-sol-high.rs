fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for line in src.lines() {
        let line = line.split_once(';').map_or(line, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if let Some(label) = tokens[0].strip_suffix(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens after label".to_string());
            }
            if !valid_label(label) {
                return Err("invalid label".to_string());
            }
            if labels.insert(label, instruction_count).is_some() {
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

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing push operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid push operand".to_string())?;
                Op::Push(value)
            }
            "jmp" | "jz" => {
                if tokens.len() < 2 {
                    return Err("missing label operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                if !valid_label(tokens[1]) {
                    return Err("invalid label operand".to_string());
                }
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                if tokens[0] == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            mnemonic => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                match mnemonic {
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
                }
            }
        };

        ops.push(op);
    }

    Ok(ops)
}
