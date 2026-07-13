fn assemble_is_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        _ => false,
    }
}

fn assemble_parse_integer(token: &str) -> Result<i64, String> {
    let bytes = token.as_bytes();
    let digits = if bytes.first() == Some(&b'-') {
        &bytes[1..]
    } else {
        bytes
    };

    if digits.is_empty() || !digits.iter().all(|byte| byte.is_ascii_digit()) {
        return Err("invalid operand".to_string());
    }

    token
        .parse::<i64>()
        .map_err(|_| "invalid operand".to_string())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        let line = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !assemble_is_label(name) {
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

    for line in src.lines() {
        let line = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                Op::Push(assemble_parse_integer(tokens[1])?)
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
