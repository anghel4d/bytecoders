fn valid_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_decimal_i64(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len() || !bytes[start..].iter().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    value.parse::<i64>().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<&str, usize>::new();
    let mut instructions = Vec::<(usize, Vec<&str>)>::new();

    for (line_index, line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let text = line.split(';').next().unwrap().trim();

        if text.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = text.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !valid_label_name(name) {
                return Err(format!("line {}: invalid label", line_number));
            }

            if labels.insert(name, instructions.len()).is_some() {
                return Err(format!("line {}: duplicate label", line_number));
            }

            continue;
        }

        if tokens[0].contains(':') {
            return Err(format!("line {}: invalid label", line_number));
        }

        instructions.push((line_number, tokens));
    }

    let mut code = Vec::with_capacity(instructions.len());

    for (line_number, tokens) in instructions {
        match tokens[0] {
            "push" => {
                if tokens.len() != 2 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                let value = match parse_decimal_i64(tokens[1]) {
                    Some(value) => value,
                    None => return Err(format!("line {}: invalid integer", line_number)),
                };

                code.push(Op::Push(value));
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Pop);
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Add);
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Mul);
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Div);
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Swap);
            }
            "jmp" | "jz" => {
                if tokens.len() != 2 || !valid_label_name(tokens[1]) {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                let target = match labels.get(tokens[1]) {
                    Some(&target) => target,
                    None => return Err(format!("line {}: undefined label", line_number)),
                };

                if tokens[0] == "jmp" {
                    code.push(Op::Jmp(target));
                } else {
                    code.push(Op::Jz(target));
                }
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Print);
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: invalid operands", line_number));
                }

                code.push(Op::Halt);
            }
            _ => return Err(format!("line {}: unknown mnemonic", line_number)),
        }
    }

    Ok(code)
}
