fn valid_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c == '_' || c.is_ascii_alphabetic() => {}
        _ => return false,
    }

    chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err(format!("line {}: trailing tokens", line_number));
            }

            let name = &tokens[0][..tokens[0].len() - 1];
            if !valid_label_name(name) {
                return Err(format!("line {}: invalid label", line_number));
            }

            if labels.insert(name.to_owned(), instruction_count).is_some() {
                return Err(format!("line {}: duplicate label", line_number));
            }

            continue;
        }

        instruction_count += 1;
    }

    let mut program = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() || tokens[0].ends_with(':') {
            continue;
        }

        let mnemonic = tokens[0];
        match mnemonic {
            "push" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }

                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| format!("line {}: invalid operand", line_number))?;
                program.push(Op::Push(value));
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Pop);
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Add);
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Mul);
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Div);
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Swap);
            }
            "jmp" | "jz" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                if !valid_label_name(tokens[1]) {
                    return Err(format!("line {}: invalid operand", line_number));
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| format!("line {}: undefined label", line_number))?;

                if mnemonic == "jmp" {
                    program.push(Op::Jmp(target));
                } else {
                    program.push(Op::Jz(target));
                }
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Print);
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                program.push(Op::Halt);
            }
            _ => {
                return Err(format!("line {}: unknown mnemonic", line_number));
            }
        }
    }

    Ok(program)
}
