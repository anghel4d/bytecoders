fn valid_label(name: &str) -> bool {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(b) if b.is_ascii_alphabetic() || b == b'_' => {}
        _ => return false,
    }

    bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn label_on_line(line: &str) -> Result<Option<&str>, String> {
    let mut tokens = line.split_whitespace();
    let Some(token) = tokens.next() else {
        return Ok(None);
    };

    if tokens.next().is_some() {
        return Ok(None);
    }

    match token.strip_suffix(':') {
        Some(name) if valid_label(name) => Ok(Some(name)),
        Some(_) => Err("invalid label".to_string()),
        None => Ok(None),
    }
}

fn parse_integer(token: &str) -> Option<i64> {
    let bytes = token.as_bytes();
    let start = usize::from(bytes.first() == Some(&b'-'));

    if start == bytes.len() || !bytes[start..].iter().all(u8::is_ascii_digit) {
        return None;
    }

    token.parse().ok()
}

fn one_operand<'a>(
    tokens: &mut std::str::SplitWhitespace<'a>,
) -> Result<&'a str, String> {
    let operand = tokens
        .next()
        .ok_or_else(|| "missing operand".to_string())?;

    if tokens.next().is_some() {
        return Err("trailing tokens".to_string());
    }

    Ok(operand)
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = label_on_line(line)? {
            if labels.insert(name.to_string(), instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut code = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        if label_on_line(line)?.is_some() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens
            .next()
            .ok_or_else(|| "missing mnemonic".to_string())?;

        match mnemonic {
            "push" => {
                let operand = one_operand(&mut tokens)?;
                let value = parse_integer(operand)
                    .ok_or_else(|| "invalid operand".to_string())?;
                code.push(Op::Push(value));
            }
            "pop" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Pop);
            }
            "add" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Add);
            }
            "sub" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Sub);
            }
            "mul" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Mul);
            }
            "div" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Div);
            }
            "mod" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Mod);
            }
            "neg" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Neg);
            }
            "dup" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Dup);
            }
            "swap" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Swap);
            }
            "jmp" => {
                let operand = one_operand(&mut tokens)?;

                if !valid_label(operand) {
                    return Err("invalid operand".to_string());
                }

                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                code.push(Op::Jmp(target));
            }
            "jz" => {
                let operand = one_operand(&mut tokens)?;

                if !valid_label(operand) {
                    return Err("invalid operand".to_string());
                }

                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                code.push(Op::Jz(target));
            }
            "print" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Print);
            }
            "halt" => {
                if tokens.next().is_some() {
                    return Err("trailing tokens".to_string());
                }
                code.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(code)
}
