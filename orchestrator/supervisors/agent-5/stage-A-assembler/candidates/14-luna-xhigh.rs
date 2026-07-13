fn assemble_label_name_is_valid(name: &str) -> bool {
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

fn assemble_parse_integer(token: &str) -> Option<i64> {
    let bytes = token.as_bytes();

    if bytes.is_empty() {
        return None;
    }

    let start = if bytes[0] == b'-' { 1 } else { 0 };

    if start == bytes.len()
        || !bytes[start..]
            .iter()
            .all(|byte| byte.is_ascii_digit())
    {
        return None;
    }

    token.parse::<i64>().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels: std::collections::HashMap<&str, usize> =
        std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !assemble_label_name_is_valid(name) {
                return Err("invalid label".to_string());
            }

            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        match tokens[0] {
            "push" => {
                if tokens.len() != 2 {
                    return Err("invalid operands".to_string());
                }

                let value = assemble_parse_integer(tokens[1])
                    .ok_or_else(|| "invalid integer".to_string())?;
                ops.push(Op::Push(value));
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Pop);
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Add);
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Mul);
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Div);
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Swap);
            }
            "jmp" => {
                if tokens.len() != 2 {
                    return Err("invalid operands".to_string());
                }

                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?;
                ops.push(Op::Jmp(target));
            }
            "jz" => {
                if tokens.len() != 2 {
                    return Err("invalid operands".to_string());
                }

                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?;
                ops.push(Op::Jz(target));
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Print);
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err("invalid operands".to_string());
                }

                ops.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(ops)
}
