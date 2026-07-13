fn valid_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_integer(token: &str) -> Result<i64, String> {
    let digits = token.strip_prefix('-').unwrap_or(token);

    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
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
        let code = line.split(';').next().unwrap_or("").trim();

        if code.is_empty() {
            continue;
        }

        let mut tokens = code.split_whitespace();
        let first = tokens.next().unwrap();

        if first.ends_with(':') {
            if tokens.next().is_some() {
                return Err("invalid label".to_string());
            }

            let name = &first[..first.len() - 1];

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
        let code = line.split(';').next().unwrap_or("").trim();

        if code.is_empty() {
            continue;
        }

        let mut tokens = code.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        if mnemonic.ends_with(':') {
            continue;
        }

        match mnemonic {
            "push" => {
                let operand = tokens
                    .next()
                    .ok_or_else(|| "missing operand".to_string())?;

                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Push(parse_integer(operand)?));
            }
            "pop" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Pop);
            }
            "add" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Add);
            }
            "sub" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Sub);
            }
            "mul" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Mul);
            }
            "div" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Div);
            }
            "mod" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Mod);
            }
            "neg" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Neg);
            }
            "dup" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Dup);
            }
            "swap" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Swap);
            }
            "jmp" | "jz" => {
                let label = tokens
                    .next()
                    .ok_or_else(|| "missing operand".to_string())?;

                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                let target = *labels
                    .get(label)
                    .ok_or_else(|| "undefined label".to_string())?;

                if mnemonic == "jmp" {
                    ops.push(Op::Jmp(target));
                } else {
                    ops.push(Op::Jz(target));
                }
            }
            "print" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Print);
            }
            "halt" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }

                ops.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(ops)
}
