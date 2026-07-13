fn is_label_name(name: &str) -> bool {
    let bytes = name.as_bytes();

    if bytes.is_empty() || !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
        return false;
    }

    bytes[1..]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
}

fn parse_integer(token: &str) -> Option<i64> {
    let digits = token.strip_prefix('-').unwrap_or(token);

    if digits.is_empty() || !digits.as_bytes().iter().all(u8::is_ascii_digit) {
        return None;
    }

    token.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !is_label_name(name) {
                return Err("invalid label".to_string());
            }

            if labels.insert(name.to_string(), instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        let mnemonic = tokens[0];

        match mnemonic {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                let value = parse_integer(tokens[1])
                    .ok_or_else(|| "invalid operand".to_string())?;
                ops.push(Op::Push(value));
            }
            "jmp" | "jz" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?;

                if mnemonic == "jmp" {
                    ops.push(Op::Jmp(target));
                } else {
                    ops.push(Op::Jz(target));
                }
            }
            "pop" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Pop);
            }
            "add" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Add);
            }
            "sub" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Mul);
            }
            "div" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Div);
            }
            "mod" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Swap);
            }
            "print" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Print);
            }
            "halt" => {
                if tokens.len() > 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(ops)
}
