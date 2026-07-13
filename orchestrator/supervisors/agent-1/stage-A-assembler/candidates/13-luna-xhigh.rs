fn valid_label(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
        return false;
    }
    bytes[1..]
        .iter()
        .all(|b| b.is_ascii_alphanumeric() || *b == b'_')
}

fn parse_decimal_i64(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return None;
    }

    let start = if bytes[0] == b'-' { 1 } else { 0 };
    if start == bytes.len()
        || !bytes[start..]
            .iter()
            .all(|b| b.is_ascii_digit())
    {
        return None;
    }

    value.parse::<i64>().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        let line = line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];
            if !valid_label(name) {
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

        if tokens[0].ends_with(':') {
            return Err("invalid label".to_string());
        }

        instruction_count += 1;
    }

    let mut output = Vec::with_capacity(instruction_count);

    for line in src.lines() {
        let line = line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                let value = parse_decimal_i64(tokens[1])
                    .ok_or_else(|| "invalid operand".to_string())?;
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
                if !valid_label(tokens[1]) {
                    return Err("invalid operand".to_string());
                }
                let target = *labels
                    .get(tokens[1])
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
                if !valid_label(tokens[1]) {
                    return Err("invalid operand".to_string());
                }
                let target = *labels
                    .get(tokens[1])
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
