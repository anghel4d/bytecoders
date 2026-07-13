fn is_label_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    match bytes.next() {
        Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') => {}
        _ => return false,
    }
    bytes.all(|byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}

fn parse_decimal(text: &str) -> Option<i64> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_index = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 1 {
            if let Some(name) = tokens[0].strip_suffix(':') {
                if !is_label_name(name) {
                    return Err(format!("line {line_number}: invalid label"));
                }
                if labels.insert(name.to_owned(), instruction_index).is_some() {
                    return Err(format!("line {line_number}: duplicate label"));
                }
                continue;
            }
        }

        instruction_index = instruction_index
            .checked_add(1)
            .ok_or_else(|| "too many instructions".to_owned())?;
    }

    let mut ops = Vec::with_capacity(instruction_index);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err(format!("line {line_number}: missing operand"));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                let value = parse_decimal(tokens[1])
                    .ok_or_else(|| format!("line {line_number}: invalid operand"))?;
                ops.push(Op::Push(value));
            }
            "jmp" | "jz" => {
                if tokens.len() < 2 {
                    return Err(format!("line {line_number}: missing operand"));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {line_number}: trailing tokens"));
                }

                let name = tokens[1];
                if !is_label_name(name) {
                    return Err(format!("line {line_number}: invalid operand"));
                }
                let target = labels
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?;

                if tokens[0] == "jmp" {
                    ops.push(Op::Jmp(target));
                } else {
                    ops.push(Op::Jz(target));
                }
            }
            mnemonic @ ("pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg"
            | "dup" | "swap" | "print" | "halt") => {
                if tokens.len() > 1 {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                ops.push(match mnemonic {
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
                    _ => unreachable!(),
                });
            }
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        }
    }

    Ok(ops)
}
