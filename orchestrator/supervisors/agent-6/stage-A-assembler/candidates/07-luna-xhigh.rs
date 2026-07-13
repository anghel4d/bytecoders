fn valid_label_name(name: &str) -> bool {
    let bytes = name.as_bytes();

    if bytes.is_empty() || !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
        return false;
    }

    bytes[1..]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
}

fn parse_decimal_integer(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    let digits = if bytes.first() == Some(&b'-') {
        if bytes.len() == 1 {
            return None;
        }
        &bytes[1..]
    } else {
        bytes
    };

    if !digits.iter().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    value.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut lines = Vec::new();

    for line in src.lines() {
        let code = line.split(';').next().unwrap().trim();
        if !code.is_empty() {
            lines.push(code.split_whitespace().collect::<Vec<_>>());
        }
    }

    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for tokens in &lines {
        let first = tokens[0];

        if first.ends_with(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }

            let name = &first[..first.len() - 1];
            if !valid_label_name(name) {
                return Err("invalid label".to_string());
            }

            if labels
                .insert(name.to_string(), instruction_count)
                .is_some()
            {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for tokens in &lines {
        let mnemonic = tokens[0];

        if mnemonic.ends_with(':') {
            continue;
        }

        let op = match mnemonic {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                let value = parse_decimal_integer(tokens[1])
                    .ok_or_else(|| "invalid operand".to_string())?;
                Op::Push(value)
            }
            "pop" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Pop
            }
            "add" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Add
            }
            "sub" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Sub
            }
            "mul" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Mul
            }
            "div" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Div
            }
            "mod" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Mod
            }
            "neg" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Neg
            }
            "dup" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Dup
            }
            "swap" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Swap
            }
            "jmp" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jmp(target)
            }
            "jz" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jz(target)
            }
            "print" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Print
            }
            "halt" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        ops.push(op);
    }

    Ok(ops)
}
