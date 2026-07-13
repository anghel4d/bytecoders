fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_integer(token: &str) -> Option<i64> {
    let digits = token.strip_prefix('-').unwrap_or(token);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    token.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_index = 0;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code);
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if let Some(name) = tokens[0].strip_suffix(':') {
            if tokens.len() != 1 {
                return Err(format!("line {}: trailing tokens", line_index + 1));
            }
            if !is_label_name(name) {
                return Err(format!("line {}: invalid label", line_index + 1));
            }
            if labels.insert(name, instruction_index).is_some() {
                return Err(format!("line {}: duplicate label", line_index + 1));
            }
        } else {
            instruction_index += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_index);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code);
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() || tokens[0].ends_with(':') {
            continue;
        }

        let line_number = line_index + 1;
        let no_operand = |op| {
            if tokens.len() == 1 {
                Ok(op)
            } else {
                Err(format!("line {line_number}: trailing tokens"))
            }
        };

        let op = match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err(format!("line {line_number}: missing operand"));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                let value = parse_integer(tokens[1])
                    .ok_or_else(|| format!("line {line_number}: invalid integer"))?;
                Op::Push(value)
            }
            "pop" => no_operand(Op::Pop)?,
            "add" => no_operand(Op::Add)?,
            "sub" => no_operand(Op::Sub)?,
            "mul" => no_operand(Op::Mul)?,
            "div" => no_operand(Op::Div)?,
            "mod" => no_operand(Op::Mod)?,
            "neg" => no_operand(Op::Neg)?,
            "dup" => no_operand(Op::Dup)?,
            "swap" => no_operand(Op::Swap)?,
            "jmp" | "jz" => {
                if tokens.len() < 2 {
                    return Err(format!("line {line_number}: missing operand"));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                if !is_label_name(tokens[1]) {
                    return Err(format!("line {line_number}: invalid label"));
                }
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?;
                if tokens[0] == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => no_operand(Op::Print)?,
            "halt" => no_operand(Op::Halt)?,
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        ops.push(op);
    }

    Ok(ops)
}
