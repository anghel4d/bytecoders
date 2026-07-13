fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_integer(token: &str) -> Result<i64, String> {
    let bytes = token.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len() || !bytes[start..].iter().all(u8::is_ascii_digit) {
        return Err("invalid integer".to_string());
    }

    token.parse::<i64>().map_err(|_| "invalid integer".to_string())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<&str, usize>::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        let code = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !valid_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, instruction_count).is_some() {
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
                    return Err("push expects one operand".to_string());
                }
                Op::Push(parse_integer(tokens[1])?)
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err("pop takes no operands".to_string());
                }
                Op::Pop
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("add takes no operands".to_string());
                }
                Op::Add
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("sub takes no operands".to_string());
                }
                Op::Sub
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("mul takes no operands".to_string());
                }
                Op::Mul
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("div takes no operands".to_string());
                }
                Op::Div
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("mod takes no operands".to_string());
                }
                Op::Mod
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("neg takes no operands".to_string());
                }
                Op::Neg
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("dup takes no operands".to_string());
                }
                Op::Dup
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err("swap takes no operands".to_string());
                }
                Op::Swap
            }
            "jmp" => {
                if tokens.len() != 2 {
                    return Err("jmp expects one operand".to_string());
                }
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jmp(target)
            }
            "jz" => {
                if tokens.len() != 2 {
                    return Err("jz expects one operand".to_string());
                }
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jz(target)
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("print takes no operands".to_string());
                }
                Op::Print
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err("halt takes no operands".to_string());
                }
                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        ops.push(op);
    }

    Ok(ops)
}
