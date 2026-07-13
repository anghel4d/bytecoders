fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn operand<'a>(
    parts: &mut std::str::SplitWhitespace<'a>,
    line: usize,
) -> Result<&'a str, String> {
    let value = parts
        .next()
        .ok_or_else(|| format!("line {}: missing operand", line))?;
    if parts.next().is_some() {
        return Err(format!("line {}: trailing tokens", line));
    }
    Ok(value)
}

fn no_operand(parts: &mut std::str::SplitWhitespace<'_>, line: usize) -> Result<(), String> {
    if parts.next().is_some() {
        return Err(format!("line {}: trailing tokens", line));
    }
    Ok(())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_index = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code);
        let mut parts = line.split_whitespace();

        let Some(first) = parts.next() else {
            continue;
        };

        if let Some(name) = first.strip_suffix(':') {
            if parts.next().is_some() || !is_label_name(name) {
                return Err(format!("line {}: invalid label", line_number));
            }
            if labels.insert(name, instruction_index).is_some() {
                return Err(format!("line {}: duplicate label", line_number));
            }
        } else {
            instruction_index += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_index);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code);
        let mut parts = line.split_whitespace();

        let Some(mnemonic) = parts.next() else {
            continue;
        };

        if mnemonic.ends_with(':') {
            continue;
        }

        let op = match mnemonic {
            "push" => {
                let value = operand(&mut parts, line_number)?
                    .parse::<i64>()
                    .map_err(|_| format!("line {}: invalid integer", line_number))?;
                Op::Push(value)
            }
            "pop" => {
                no_operand(&mut parts, line_number)?;
                Op::Pop
            }
            "add" => {
                no_operand(&mut parts, line_number)?;
                Op::Add
            }
            "sub" => {
                no_operand(&mut parts, line_number)?;
                Op::Sub
            }
            "mul" => {
                no_operand(&mut parts, line_number)?;
                Op::Mul
            }
            "div" => {
                no_operand(&mut parts, line_number)?;
                Op::Div
            }
            "mod" => {
                no_operand(&mut parts, line_number)?;
                Op::Mod
            }
            "neg" => {
                no_operand(&mut parts, line_number)?;
                Op::Neg
            }
            "dup" => {
                no_operand(&mut parts, line_number)?;
                Op::Dup
            }
            "swap" => {
                no_operand(&mut parts, line_number)?;
                Op::Swap
            }
            "jmp" | "jz" => {
                let name = operand(&mut parts, line_number)?;
                if !is_label_name(name) {
                    return Err(format!("line {}: invalid label", line_number));
                }
                let target = labels
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("line {}: undefined label", line_number))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => {
                no_operand(&mut parts, line_number)?;
                Op::Print
            }
            "halt" => {
                no_operand(&mut parts, line_number)?;
                Op::Halt
            }
            _ => return Err(format!("line {}: unknown mnemonic", line_number)),
        };

        ops.push(op);
    }

    Ok(ops)
}
