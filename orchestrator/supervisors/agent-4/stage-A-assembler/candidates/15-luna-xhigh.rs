pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instructions = Vec::<(usize, String)>::new();

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let code = match raw_line.find(';') {
            Some(comment_start) => &raw_line[..comment_start],
            None => raw_line,
        };
        let line = code.trim();

        if line.is_empty() {
            continue;
        }

        if line.ends_with(':') {
            let name = &line[..line.len() - 1];
            if !is_assembly_label(name) {
                return Err(format!("line {}: invalid label", line_number));
            }
            if labels
                .insert(name.to_string(), instructions.len())
                .is_some()
            {
                return Err(format!("line {}: duplicate label", line_number));
            }
        } else {
            instructions.push((line_number, line.to_string()));
        }
    }

    let mut result = Vec::with_capacity(instructions.len());

    for (line_number, line) in instructions {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = tokens[0];

        match mnemonic {
            "push" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                let value = match tokens[1].parse::<i64>() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("line {}: invalid operand", line_number)),
                };
                result.push(Op::Push(value));
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Pop);
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Add);
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Mul);
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Div);
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Swap);
            }
            "jmp" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                let target = match labels.get(tokens[1]) {
                    Some(&target) => target,
                    None => return Err(format!("line {}: undefined label", line_number)),
                };
                result.push(Op::Jmp(target));
            }
            "jz" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                let target = match labels.get(tokens[1]) {
                    Some(&target) => target,
                    None => return Err(format!("line {}: undefined label", line_number)),
                };
                result.push(Op::Jz(target));
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Print);
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                result.push(Op::Halt);
            }
            _ => return Err(format!("line {}: unknown mnemonic", line_number)),
        }
    }

    Ok(result)
}

fn is_assembly_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
        _ => return false,
    }

    chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
}
