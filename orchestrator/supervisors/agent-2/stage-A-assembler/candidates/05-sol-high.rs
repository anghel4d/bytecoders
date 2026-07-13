fn valid_label(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'_'))
        && bytes.all(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}

fn valid_integer(value: &str) -> bool {
    let digits = value.strip_prefix('-').unwrap_or(value);
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for (line_number, source_line) in src.lines().enumerate() {
        let line = source_line
            .split_once(';')
            .map_or(source_line, |(code, _)| code)
            .trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !valid_label(name) {
                return Err(format!("line {}: invalid label", line_number + 1));
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err(format!("line {}: duplicate label", line_number + 1));
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_number, source_line) in src.lines().enumerate() {
        let line = source_line
            .split_once(';')
            .map_or(source_line, |(code, _)| code)
            .trim();

        if line.is_empty() || line.strip_suffix(':').is_some_and(valid_label) {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = tokens[0];
        let operand = match tokens.len() {
            1 => None,
            2 => Some(tokens[1]),
            _ => return Err(format!("line {}: trailing tokens", line_number + 1)),
        };

        let op = match mnemonic {
            "push" => {
                let value = operand.ok_or_else(|| {
                    format!("line {}: missing operand for push", line_number + 1)
                })?;
                if !valid_integer(value) {
                    return Err(format!("line {}: invalid push operand", line_number + 1));
                }
                Op::Push(
                    value
                        .parse()
                        .map_err(|_| format!("line {}: invalid push operand", line_number + 1))?,
                )
            }
            "jmp" | "jz" => {
                let name = operand.ok_or_else(|| {
                    format!("line {}: missing operand for {}", line_number + 1, mnemonic)
                })?;
                if !valid_label(name) {
                    return Err(format!("line {}: invalid label operand", line_number + 1));
                }
                let target = labels.get(name).copied().ok_or_else(|| {
                    format!("line {}: undefined label {}", line_number + 1, name)
                })?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "pop" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Pop
            }
            "add" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Add
            }
            "sub" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Sub
            }
            "mul" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Mul
            }
            "div" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Div
            }
            "mod" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Mod
            }
            "neg" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Neg
            }
            "dup" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Dup
            }
            "swap" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Swap
            }
            "print" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Print
            }
            "halt" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Halt
            }
            _ => {
                return Err(format!(
                    "line {}: unknown mnemonic {}",
                    line_number + 1,
                    mnemonic
                ));
            }
        };

        ops.push(op);
    }

    Ok(ops)
}
