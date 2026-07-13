fn is_label_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b) if b.is_ascii_alphabetic() || b == b'_')
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn parse_decimal_i64(token: &str) -> Option<i64> {
    let digits = token.strip_prefix('-').unwrap_or(token);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    token.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_index = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !is_label_name(name) {
                return Err(format!("line {}: invalid label", line_index + 1));
            }
            if labels.insert(name, instruction_index).is_some() {
                return Err(format!("line {}: duplicate label", line_index + 1));
            }
        } else {
            instruction_index = instruction_index
                .checked_add(1)
                .ok_or_else(|| "too many instructions".to_string())?;
        }
    }

    let mut ops = Vec::with_capacity(instruction_index);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        let op = match mnemonic {
            "push" => {
                let operand = tokens
                    .next()
                    .ok_or_else(|| format!("line {}: missing operand", line_index + 1))?;
                let value = parse_decimal_i64(operand)
                    .ok_or_else(|| format!("line {}: invalid integer", line_index + 1))?;
                Op::Push(value)
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
            "jmp" | "jz" => {
                let operand = tokens
                    .next()
                    .ok_or_else(|| format!("line {}: missing operand", line_index + 1))?;
                if !is_label_name(operand) {
                    return Err(format!("line {}: invalid label", line_index + 1));
                }
                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| format!("line {}: undefined label", line_index + 1))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => return Err(format!("line {}: unknown mnemonic", line_index + 1)),
        };

        if tokens.next().is_some() {
            return Err(format!("line {}: trailing tokens", line_index + 1));
        }

        ops.push(op);
    }

    Ok(ops)
}
