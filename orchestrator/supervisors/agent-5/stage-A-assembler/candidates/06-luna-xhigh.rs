fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut lines = Vec::<Vec<&str>>::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let code = raw_line.split(';').next().unwrap_or("").trim();
        if code.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }

            let name = tokens[0]
                .strip_suffix(':')
                .ok_or_else(|| "invalid label".to_string())?;

            if !is_label_name(name) {
                return Err("invalid label".to_string());
            }

            if labels
                .insert(name.to_string(), instruction_count)
                .is_some()
            {
                return Err("duplicate label".to_string());
            }

            continue;
        }

        lines.push(tokens);
        instruction_count += 1;
    }

    let mut output = Vec::with_capacity(instruction_count);

    for tokens in lines {
        match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;
                output.push(Op::Push(value));
            }
            "pop" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Pop);
            }
            "add" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Add);
            }
            "sub" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Mul);
            }
            "div" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Div);
            }
            "mod" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Swap);
            }
            "jmp" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                if !is_label_name(tokens[1]) {
                    return Err("invalid operand".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                output.push(Op::Jmp(target));
            }
            "jz" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                if !is_label_name(tokens[1]) {
                    return Err("invalid operand".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                output.push(Op::Jz(target));
            }
            "print" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Print);
            }
            "halt" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(output)
}
