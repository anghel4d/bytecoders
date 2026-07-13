fn valid_label_name(name: &str) -> bool {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(byte) if byte == b'_' || byte.is_ascii_alphabetic() => {}
        _ => return false,
    }

    bytes.all(|byte| byte == b'_' || byte.is_ascii_alphanumeric())
}

fn parse_decimal_i64(token: &str) -> Option<i64> {
    let bytes = token.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len() || bytes[start..].iter().any(|byte| !byte.is_ascii_digit()) {
        return None;
    }

    token.parse().ok()
}

fn resolve_label(labels: &[(String, usize)], name: &str) -> Option<usize> {
    labels
        .iter()
        .find(|(label, _)| label == name)
        .map(|(_, index)| *index)
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut lines = Vec::new();

    for line in src.lines() {
        let code = line.split_once(';').map_or(line, |(code, _)| code);
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if !tokens.is_empty() {
            lines.push(tokens);
        }
    }

    let mut labels = Vec::new();
    let mut instruction_count = 0;

    for tokens in &lines {
        if tokens.len() == 1 {
            if let Some(name) = tokens[0].strip_suffix(':') {
                if !valid_label_name(name) {
                    return Err("invalid label".to_string());
                }

                if labels.iter().any(|(label, _)| label == name) {
                    return Err("duplicate label".to_string());
                }

                labels.push((name.to_string(), instruction_count));
                continue;
            }
        }

        instruction_count += 1;
    }

    let mut output = Vec::with_capacity(instruction_count);

    for tokens in lines {
        if tokens.len() == 1
            && tokens[0]
                .strip_suffix(':')
                .is_some_and(valid_label_name)
        {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                if tokens.len() != 2 {
                    return Err("invalid push".to_string());
                }

                let value = parse_decimal_i64(tokens[1])
                    .ok_or_else(|| "invalid operand".to_string())?;

                Op::Push(value)
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err("invalid pop".to_string());
                }

                Op::Pop
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("invalid add".to_string());
                }

                Op::Add
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("invalid sub".to_string());
                }

                Op::Sub
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("invalid mul".to_string());
                }

                Op::Mul
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("invalid div".to_string());
                }

                Op::Div
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("invalid mod".to_string());
                }

                Op::Mod
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("invalid neg".to_string());
                }

                Op::Neg
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("invalid dup".to_string());
                }

                Op::Dup
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err("invalid swap".to_string());
                }

                Op::Swap
            }
            "jmp" => {
                if tokens.len() != 2 {
                    return Err("invalid jmp".to_string());
                }

                let target = resolve_label(&labels, tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?;

                Op::Jmp(target)
            }
            "jz" => {
                if tokens.len() != 2 {
                    return Err("invalid jz".to_string());
                }

                let target = resolve_label(&labels, tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?;

                Op::Jz(target)
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("invalid print".to_string());
                }

                Op::Print
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err("invalid halt".to_string());
                }

                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        output.push(op);
    }

    Ok(output)
}
