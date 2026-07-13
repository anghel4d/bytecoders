fn valid_label_name(name: &str) -> bool {
    let mut chars = name.bytes();

    match chars.next() {
        Some(b) if b.is_ascii_alphabetic() || b == b'_' => {}
        _ => return false,
    }

    chars.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        let code = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !valid_label_name(name) {
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

    for line in src.lines() {
        let code = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }
        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                if tokens.len() != 2 {
                    return Err("invalid push operand".to_string());
                }
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid push operand".to_string())?;
                Op::Push(value)
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Pop
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Add
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Sub
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Mul
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Div
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Mod
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Neg
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Dup
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Swap
            }
            "jmp" => {
                if tokens.len() != 2 {
                    return Err("invalid jmp operand".to_string());
                }
                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jmp(target)
            }
            "jz" => {
                if tokens.len() != 2 {
                    return Err("invalid jz operand".to_string());
                }
                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jz(target)
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                Op::Print
            }
            "halt" => {
                if tokens.len() != 1 {
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
