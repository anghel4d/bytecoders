fn valid_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instruction_index = 0usize;

    for (line_number, raw_line) in src.lines().enumerate() {
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !valid_label_name(name) {
                return Err(format!("line {}: invalid label", line_number + 1));
            }
            if labels.contains_key(name) {
                return Err(format!("line {}: duplicate label", line_number + 1));
            }

            labels.insert(name.to_owned(), instruction_index);
        } else {
            instruction_index += 1;
        }
    }

    let mut output = Vec::new();

    for (line_number, raw_line) in src.lines().enumerate() {
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number + 1));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }

                match tokens[1].parse::<i64>() {
                    Ok(value) => Op::Push(value),
                    Err(_) => {
                        return Err(format!("line {}: invalid integer", line_number + 1));
                    }
                }
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Pop
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Add
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Sub
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Mul
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Div
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Mod
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Neg
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Dup
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Swap
            }
            "jmp" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number + 1));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }

                match labels.get(tokens[1]).copied() {
                    Some(target) => Op::Jmp(target),
                    None => return Err(format!("line {}: undefined label", line_number + 1)),
                }
            }
            "jz" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number + 1));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }

                match labels.get(tokens[1]).copied() {
                    Some(target) => Op::Jz(target),
                    None => return Err(format!("line {}: undefined label", line_number + 1)),
                }
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Print
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number + 1));
                }
                Op::Halt
            }
            _ => return Err(format!("line {}: unknown mnemonic", line_number + 1)),
        };

        output.push(op);
    }

    Ok(output)
}
