fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some('A'..='Z' | 'a'..='z' | '_') => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !is_label_name(name) {
                return Err(format!("line {line_number}: invalid label"));
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
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = tokens[0];

        let no_operand = || {
            if tokens.len() > 1 {
                Err(format!("line {line_number}: trailing tokens"))
            } else {
                Ok(())
            }
        };

        let operand = || {
            if tokens.len() < 2 {
                Err(format!("line {line_number}: missing operand"))
            } else if tokens.len() > 2 {
                Err(format!("line {line_number}: trailing tokens"))
            } else {
                Ok(tokens[1])
            }
        };

        let op = match mnemonic {
            "push" => {
                let value = operand()?
                    .parse::<i64>()
                    .map_err(|_| format!("line {line_number}: invalid integer"))?;
                Op::Push(value)
            }
            "pop" => {
                no_operand()?;
                Op::Pop
            }
            "add" => {
                no_operand()?;
                Op::Add
            }
            "sub" => {
                no_operand()?;
                Op::Sub
            }
            "mul" => {
                no_operand()?;
                Op::Mul
            }
            "div" => {
                no_operand()?;
                Op::Div
            }
            "mod" => {
                no_operand()?;
                Op::Mod
            }
            "neg" => {
                no_operand()?;
                Op::Neg
            }
            "dup" => {
                no_operand()?;
                Op::Dup
            }
            "swap" => {
                no_operand()?;
                Op::Swap
            }
            "jmp" | "jz" => {
                let name = operand()?;
                if !is_label_name(name) {
                    return Err(format!("line {line_number}: invalid label"));
                }
                let target = labels
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?;

                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => {
                no_operand()?;
                Op::Print
            }
            "halt" => {
                no_operand()?;
                Op::Halt
            }
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        ops.push(op);
    }

    Ok(ops)
}
