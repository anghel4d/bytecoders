fn assemble_label_is_valid(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (line_number, raw_line) in src.lines().enumerate() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];
            if !assemble_label_is_valid(name) {
                return Err(format!("line {}: invalid label", line_number + 1));
            }
            if labels.insert(name.to_string(), instruction_count).is_some() {
                return Err(format!("line {}: duplicate label", line_number + 1));
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut output = Vec::with_capacity(instruction_count);

    for (line_number, raw_line) in src.lines().enumerate() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        let mnemonic = tokens[0];
        let line_error = |message: &str| Err(format!("line {}: {}", line_number + 1, message));

        match mnemonic {
            "push" => {
                if tokens.len() < 2 {
                    return line_error("missing operand");
                }
                if tokens.len() > 2 {
                    return line_error("trailing tokens");
                }
                let value = match tokens[1].parse::<i64>() {
                    Ok(value) => value,
                    Err(_) => return line_error("invalid operand"),
                };
                output.push(Op::Push(value));
            }
            "pop" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Pop);
            }
            "add" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Add);
            }
            "sub" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Mul);
            }
            "div" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Div);
            }
            "mod" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Swap);
            }
            "jmp" => {
                if tokens.len() < 2 {
                    return line_error("missing operand");
                }
                if tokens.len() > 2 {
                    return line_error("trailing tokens");
                }
                let target = match labels.get(tokens[1]) {
                    Some(&target) => target,
                    None => return line_error("undefined label"),
                };
                output.push(Op::Jmp(target));
            }
            "jz" => {
                if tokens.len() < 2 {
                    return line_error("missing operand");
                }
                if tokens.len() > 2 {
                    return line_error("trailing tokens");
                }
                let target = match labels.get(tokens[1]) {
                    Some(&target) => target,
                    None => return line_error("undefined label"),
                };
                output.push(Op::Jz(target));
            }
            "print" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Print);
            }
            "halt" => {
                if tokens.len() != 1 {
                    return line_error("trailing tokens");
                }
                output.push(Op::Halt);
            }
            _ => return line_error("unknown mnemonic"),
        }
    }

    Ok(output)
}
