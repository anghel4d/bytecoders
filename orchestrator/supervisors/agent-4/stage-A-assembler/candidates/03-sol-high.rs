fn assembler_label(line: &str) -> Option<&str> {
    let mut tokens = line.split_whitespace();
    let token = tokens.next()?;

    if tokens.next().is_none() {
        token.strip_suffix(':')
    } else {
        None
    }
}

fn valid_assembler_label(name: &str) -> bool {
    let mut chars = name.chars();

    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_'))
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();

        if line.is_empty() {
            continue;
        }

        if let Some(label) = assembler_label(line) {
            if !valid_assembler_label(label) {
                return Err(format!("line {line_number}: invalid label"));
            }
            if labels.insert(label, instruction_count).is_some() {
                return Err(format!("line {line_number}: duplicate label"));
            }
        } else {
            instruction_count = instruction_count
                .checked_add(1)
                .ok_or_else(|| "too many instructions".to_string())?;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();

        if line.is_empty() || assembler_label(line).is_some() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();
        let operand = tokens.next();
        let trailing = tokens.next().is_some();

        let no_operand = |op| {
            if operand.is_some() {
                Err(format!("line {line_number}: trailing tokens"))
            } else {
                Ok(op)
            }
        };

        let op = match mnemonic {
            "push" => {
                let value = operand
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?;
                if trailing {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                let value = value
                    .parse::<i64>()
                    .map_err(|_| format!("line {line_number}: invalid integer"))?;
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
                let label = operand
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?;
                if trailing {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                if !valid_assembler_label(label) {
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
            "print" => no_operand(Op::Print)?,
            "halt" => no_operand(Op::Halt)?,
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        ops.push(op);
    }

    Ok(ops)
}
