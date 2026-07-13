fn ano_is_asm_label_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c == '_' || c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

fn ano_parse_asm_i64(token: &str) -> Result<i64, String> {
    let digits = if token.starts_with('-') {
        &token[1..]
    } else {
        token
    };

    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err("invalid integer".to_string());
    }

    token.parse::<i64>().map_err(|_| "invalid integer".to_string())
}

fn ano_validate_asm_tokens(tokens: &[&str]) -> Result<(), String> {
    match tokens[0] {
        "push" => {
            if tokens.len() < 2 {
                Err("missing operand".to_string())
            } else if tokens.len() > 2 {
                Err("trailing tokens".to_string())
            } else {
                ano_parse_asm_i64(tokens[1]).map(|_| ())
            }
        }
        "jmp" | "jz" => {
            if tokens.len() < 2 {
                Err("missing operand".to_string())
            } else if tokens.len() > 2 {
                Err("trailing tokens".to_string())
            } else if ano_is_asm_label_name(tokens[1]) {
                Ok(())
            } else {
                Err("invalid label".to_string())
            }
        }
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
        | "print" | "halt" => {
            if tokens.len() > 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(())
            }
        }
        _ => Err("unknown mnemonic".to_string()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<&str, usize>::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];
            if !ano_is_asm_label_name(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
            continue;
        }

        ano_validate_asm_tokens(&tokens)?;
        instruction_count += 1;
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
            "push" => ops.push(Op::Push(ano_parse_asm_i64(tokens[1])?)),
            "pop" => ops.push(Op::Pop),
            "add" => ops.push(Op::Add),
            "sub" => ops.push(Op::Sub),
            "mul" => ops.push(Op::Mul),
            "div" => ops.push(Op::Div),
            "mod" => ops.push(Op::Mod),
            "neg" => ops.push(Op::Neg),
            "dup" => ops.push(Op::Dup),
            "swap" => ops.push(Op::Swap),
            "jmp" => {
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                ops.push(Op::Jmp(target));
            }
            "jz" => {
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                ops.push(Op::Jz(target));
            }
            "print" => ops.push(Op::Print),
            "halt" => ops.push(Op::Halt),
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(ops)
}
