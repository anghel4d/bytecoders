fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code);
        let mut tokens = line.split_whitespace();
        let Some(first) = tokens.next() else {
            continue;
        };

        if let Some(name) = first.strip_suffix(':') {
            if !is_label_name(name) {
                return Err(format!("line {line_number}: invalid label"));
            }
            if tokens.next().is_some() {
                return Err(format!("line {line_number}: trailing tokens"));
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err(format!("line {line_number}: duplicate label"));
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code);
        let mut tokens = line.split_whitespace();
        let Some(mnemonic) = tokens.next() else {
            continue;
        };

        if mnemonic.ends_with(':') {
            continue;
        }

        let operand = tokens.next();
        if tokens.next().is_some() {
            return Err(format!("line {line_number}: trailing tokens"));
        }

        let op = match mnemonic {
            "push" => {
                let value = operand
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?
                    .parse::<i64>()
                    .map_err(|_| format!("line {line_number}: invalid integer"))?;
                Op::Push(value)
            }
            "jmp" | "jz" => {
                let label =
                    operand.ok_or_else(|| format!("line {line_number}: missing operand"))?;
                if !is_label_name(label) {
                    return Err(format!("line {line_number}: invalid label"));
                }
                let target = labels
                    .get(label)
                    .copied()
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "pop" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Pop
            }
            "add" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Add
            }
            "sub" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Sub
            }
            "mul" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Mul
            }
            "div" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Div
            }
            "mod" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Mod
            }
            "neg" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Neg
            }
            "dup" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Dup
            }
            "swap" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Swap
            }
            "print" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Print
            }
            "halt" => {
                if operand.is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Halt
            }
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        ops.push(op);
    }

    Ok(ops)
}
