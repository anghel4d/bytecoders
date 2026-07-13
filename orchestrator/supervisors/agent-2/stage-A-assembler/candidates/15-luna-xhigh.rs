pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    fn valid_label(name: &str) -> bool {
        let bytes = name.as_bytes();
        if bytes.is_empty() {
            return false;
        }

        let first = bytes[0];
        if !(first == b'_' || first.is_ascii_alphabetic()) {
            return false;
        }

        bytes[1..]
            .iter()
            .all(|byte| *byte == b'_' || byte.is_ascii_alphanumeric())
    }

    fn parse_integer(value: &str) -> Option<i64> {
        let bytes = value.as_bytes();
        let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

        if start == bytes.len()
            || !bytes[start..]
                .iter()
                .all(|byte| byte.is_ascii_digit())
        {
            return None;
        }

        value.parse::<i64>().ok()
    }

    let mut labels = std::collections::HashMap::new();
    let mut statements = Vec::new();
    let mut instruction_count = 0;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("invalid label".to_string());
            }

            let name = &tokens[0][..tokens[0].len() - 1];
            if !valid_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            statements.push(tokens);
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for tokens in statements {
        let mnemonic = tokens[0];
        let op = match mnemonic {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                let value = parse_integer(tokens[1])
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
