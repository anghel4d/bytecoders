fn valid_label(name: &str) -> bool {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(first) if first.is_ascii_alphabetic() || first == b'_' => {
            bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        }
        _ => false,
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<&str, usize>::new();
    let mut lines = Vec::new();
    let mut instruction_index = 0;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !valid_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, instruction_index).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            if tokens[0].ends_with(':') {
                return Err("trailing tokens".to_string());
            }
            lines.push(tokens);
            instruction_index += 1;
        }
    }

    let mut output = Vec::with_capacity(instruction_index);

    for tokens in lines {
        let mnemonic = tokens[0];
        let op = match mnemonic {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                Op::Push(
                    tokens[1]
                        .parse::<i64>()
                        .map_err(|_| "invalid operand".to_string())?,
                )
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
                if !valid_label(tokens[1]) {
                    return Err("invalid label".to_string());
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
                if !valid_label(tokens[1]) {
                    return Err("invalid label".to_string());
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

        output.push(op);
    }

    Ok(output)
}
