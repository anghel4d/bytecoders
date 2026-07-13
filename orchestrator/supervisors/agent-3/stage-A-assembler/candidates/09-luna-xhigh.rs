fn valid_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_decimal_i64(token: &str) -> Result<i64, ()> {
    let digits = token.strip_prefix('-').unwrap_or(token);

    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(());
    }

    token.parse::<i64>().map_err(|_| ())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !valid_label_name(name) {
                return Err(format!("line {}: invalid label", line_number));
            }

            if labels.insert(name.to_string(), instruction_count).is_some() {
                return Err(format!("line {}: duplicate label", line_number));
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut code = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
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
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }

                let value = parse_decimal_i64(tokens[1])
                    .map_err(|_| format!("line {}: invalid integer", line_number))?;
                Op::Push(value)
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Pop
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Add
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Sub
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Mul
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Div
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Mod
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Neg
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Dup
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Swap
            }
            "jmp" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                if !valid_label_name(tokens[1]) {
                    return Err(format!("line {}: invalid label", line_number));
                }

                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| format!("line {}: undefined label", line_number))?;
                Op::Jmp(target)
            }
            "jz" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                if !valid_label_name(tokens[1]) {
                    return Err(format!("line {}: invalid label", line_number));
                }

                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| format!("line {}: undefined label", line_number))?;
                Op::Jz(target)
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Print
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                Op::Halt
            }
            _ => return Err(format!("line {}: unknown mnemonic", line_number)),
        };

        code.push(op);
    }

    Ok(code)
}
