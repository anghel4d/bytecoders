fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_integer(token: &str) -> Result<i64, String> {
    let bytes = token.as_bytes();
    let digits = if bytes.first() == Some(&b'-') {
        &bytes[1..]
    } else {
        bytes
    };

    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) {
        return Err("invalid operand".to_string());
    }

    token
        .parse::<i64>()
        .map_err(|_| "invalid operand".to_string())
}

fn require_no_operand(tokens: &[&str]) -> Result<(), String> {
    if tokens.len() == 1 {
        Ok(())
    } else {
        Err("trailing tokens".to_string())
    }
}

fn resolve_label(
    labels: &std::collections::HashMap<String, usize>,
    name: &str,
) -> Result<usize, String> {
    if !is_label_name(name) {
        return Err("invalid label".to_string());
    }

    labels
        .get(name)
        .copied()
        .ok_or_else(|| "undefined label".to_string())
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

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();

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

                ops.push(Op::Push(parse_integer(tokens[1])?));
            }
            "pop" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Pop);
            }
            "add" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Add);
            }
            "sub" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Sub);
            }
            "mul" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Mul);
            }
            "div" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Div);
            }
            "mod" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Mod);
            }
            "neg" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Neg);
            }
            "dup" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Dup);
            }
            "swap" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Swap);
            }
            "jmp" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Jmp(resolve_label(&labels, tokens[1])?));
            }
            "jz" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Jz(resolve_label(&labels, tokens[1])?));
            }
            "print" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Print);
            }
            "halt" => {
                require_no_operand(&tokens)?;
                ops.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(ops)
}
