fn assemble_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn assemble_integer(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len() || !bytes[start..].iter().all(|b| b.is_ascii_digit()) {
        return None;
    }

    value.parse::<i64>().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut lines = Vec::<Vec<&str>>::new();

    for raw_line in src.lines() {
        let code = raw_line.split(';').next().unwrap_or("");
        let tokens = code.split_whitespace().collect::<Vec<_>>();

        if tokens.is_empty() {
            continue;
        }

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("invalid label declaration".to_string());
            }

            let name = &tokens[0][..tokens[0].len() - 1];
            if !assemble_label_name(name) {
                return Err("invalid label".to_string());
            }

            if labels.insert(name.to_string(), lines.len()).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            lines.push(tokens);
        }
    }

    let mut output = Vec::with_capacity(lines.len());

    for tokens in lines {
        let mnemonic = tokens[0];

        match mnemonic {
            "push" => {
                if tokens.len() != 2 {
                    return Err("invalid operands for push".to_string());
                }

                let value = assemble_integer(tokens[1])
                    .ok_or_else(|| "invalid operands for push".to_string())?;
                output.push(Op::Push(value));
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for pop".to_string());
                }
                output.push(Op::Pop);
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for add".to_string());
                }
                output.push(Op::Add);
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for sub".to_string());
                }
                output.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for mul".to_string());
                }
                output.push(Op::Mul);
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for div".to_string());
                }
                output.push(Op::Div);
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for mod".to_string());
                }
                output.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for neg".to_string());
                }
                output.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for dup".to_string());
                }
                output.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for swap".to_string());
                }
                output.push(Op::Swap);
            }
            "jmp" => {
                if tokens.len() != 2 || !assemble_label_name(tokens[1]) {
                    return Err("invalid operands for jmp".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| format!("undefined label: {}", tokens[1]))?;
                output.push(Op::Jmp(target));
            }
            "jz" => {
                if tokens.len() != 2 || !assemble_label_name(tokens[1]) {
                    return Err("invalid operands for jz".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| format!("undefined label: {}", tokens[1]))?;
                output.push(Op::Jz(target));
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for print".to_string());
                }
                output.push(Op::Print);
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err("invalid operands for halt".to_string());
                }
                output.push(Op::Halt);
            }
            _ => return Err(format!("unknown mnemonic: {}", mnemonic)),
        }
    }

    Ok(output)
}
