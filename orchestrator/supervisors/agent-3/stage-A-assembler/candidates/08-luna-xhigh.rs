fn assemble_label_line<'a>(tokens: &[&'a str]) -> Result<Option<&'a str>, String> {
    if tokens.is_empty() || !tokens[0].ends_with(':') {
        return Ok(None);
    }
    if tokens.len() != 1 {
        return Err("trailing tokens".to_string());
    }

    let name = &tokens[0][..tokens[0].len() - 1];
    if !assemble_valid_label(name) {
        return Err("invalid label".to_string());
    }

    Ok(Some(name))
}

fn assemble_valid_label(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty()
        || !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_')
    {
        return false;
    }

    bytes[1..]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
}

fn assemble_decimal_int(value: &str) -> Result<i64, String> {
    let bytes = value.as_bytes();
    let digits = if bytes.first() == Some(&b'-') {
        &bytes[1..]
    } else {
        bytes
    };

    if digits.is_empty() || !digits.iter().all(|byte| byte.is_ascii_digit()) {
        return Err("invalid operand".to_string());
    }

    value
        .parse::<i64>()
        .map_err(|_| "invalid operand".to_string())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for line in src.lines() {
        let code = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if let Some(name) = assemble_label_line(&tokens)? {
            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else if !tokens.is_empty() {
            instruction_count += 1;
        }
    }

    let mut output = Vec::with_capacity(instruction_count);

    for line in src.lines() {
        let code = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }
        if assemble_label_line(&tokens)?.is_some() {
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
                output.push(Op::Push(assemble_decimal_int(tokens[1])?));
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err(if tokens.len() < 2 {
                        "missing operand".to_string()
                    } else {
                        "trailing tokens".to_string()
                    });
                }
                output.push(Op::Pop);
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Add);
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Mul);
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Div);
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() != 1 {
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
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                output.push(Op::Jz(target));
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Print);
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(output)
}
