fn assemble_tokens(line: &str) -> Vec<&str> {
    line.split(';')
        .next()
        .unwrap_or("")
        .split_whitespace()
        .collect()
}

fn assemble_is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn assemble_integer(text: &str) -> Option<i64> {
    let bytes = text.as_bytes();
    let digits = if bytes.first() == Some(&b'-') {
        &bytes[1..]
    } else {
        bytes
    };

    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }

    text.parse::<i64>().ok()
}

fn assemble_one_operand<'a>(tokens: &[&'a str]) -> Result<&'a str, String> {
    match tokens.len() {
        1 => Err("missing operand".to_string()),
        2 => Ok(tokens[1]),
        _ => Err("trailing tokens".to_string()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels: std::collections::HashMap<&str, usize> =
        std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        let tokens = assemble_tokens(line);

        if tokens.is_empty() {
            continue;
        }

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }

            let name = &tokens[0][..tokens[0].len() - 1];

            if !assemble_is_label_name(name) {
                return Err("invalid label".to_string());
            }

            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count = instruction_count
                .checked_add(1)
                .ok_or_else(|| "too many instructions".to_string())?;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for line in src.lines() {
        let tokens = assemble_tokens(line);

        if tokens.is_empty() {
            continue;
        }

        if tokens[0].ends_with(':') {
            continue;
        }

        match tokens[0] {
            "push" => {
                let operand = assemble_one_operand(&tokens)?;
                let value = assemble_integer(operand)
                    .ok_or_else(|| "invalid integer".to_string())?;
                ops.push(Op::Push(value));
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Pop);
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Add);
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Mul);
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Div);
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Swap);
            }
            "jmp" => {
                let operand = assemble_one_operand(&tokens)?;

                if !assemble_is_label_name(operand) {
                    return Err("invalid label".to_string());
                }

                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                ops.push(Op::Jmp(target));
            }
            "jz" => {
                let operand = assemble_one_operand(&tokens)?;

                if !assemble_is_label_name(operand) {
                    return Err("invalid label".to_string());
                }

                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                ops.push(Op::Jz(target));
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Print);
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                ops.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(ops)
}
