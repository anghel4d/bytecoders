fn valid_label(name: &str) -> bool {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(b'A'..=b'Z') | Some(b'a'..=b'z') | Some(b'_') => {}
        _ => return false,
    }

    bytes.all(|byte| {
        matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_')
    })
}

fn parse_assembly_integer(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len() || !bytes[start..].iter().all(u8::is_ascii_digit) {
        return None;
    }

    value.parse().ok()
}

fn resolve_assembly_label(
    labels: &std::collections::HashMap<String, usize>,
    name: &str,
) -> Result<usize, String> {
    if !valid_label(name) {
        return Err("invalid label".to_string());
    }

    labels
        .get(name)
        .copied()
        .ok_or_else(|| "undefined label".to_string())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instructions = Vec::new();
    let mut next_index = 0;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !valid_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name.to_string(), next_index).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instructions.push(tokens);
            next_index += 1;
        }
    }

    let mut output = Vec::with_capacity(instructions.len());

    for tokens in instructions {
        match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                let value = parse_assembly_integer(tokens[1])
                    .ok_or_else(|| "invalid integer".to_string())?;
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

                let target = resolve_assembly_label(&labels, tokens[1])?;
                output.push(Op::Jmp(target));
            }
            "jz" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                let target = resolve_assembly_label(&labels, tokens[1])?;
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
