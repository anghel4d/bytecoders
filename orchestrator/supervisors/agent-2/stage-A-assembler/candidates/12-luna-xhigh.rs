fn assemble_label_is_valid(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn assemble_parse_int(text: &str) -> Result<i64, String> {
    let bytes = text.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len() || !bytes[start..].iter().all(u8::is_ascii_digit) {
        return Err("invalid integer".to_string());
    }

    text.parse::<i64>()
        .map_err(|_| "invalid integer".to_string())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instructions = Vec::new();

    for raw_line in src.lines() {
        let line = raw_line
            .split_once(';')
            .map_or(raw_line, |(code, _)| code)
            .trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !assemble_label_is_valid(name) {
                return Err("invalid label".to_string());
            }

            if labels.insert(name, instructions.len()).is_some() {
                return Err("duplicate label".to_string());
            }

            continue;
        }

        instructions.push(tokens);
    }

    let mut output = Vec::with_capacity(instructions.len());

    for tokens in instructions {
        match tokens[0] {
            "push" => {
                if tokens.len() != 2 {
                    return Err("invalid push operand".to_string());
                }
                output.push(Op::Push(assemble_parse_int(tokens[1])?));
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Pop);
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Add);
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Sub);
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Mul);
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Div);
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Mod);
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Neg);
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Dup);
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Swap);
            }
            "jmp" | "jz" => {
                if tokens.len() != 2 {
                    return Err("invalid jump operand".to_string());
                }
                if !assemble_label_is_valid(tokens[1]) {
                    return Err("invalid label".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                if tokens[0] == "jmp" {
                    output.push(Op::Jmp(target));
                } else {
                    output.push(Op::Jz(target));
                }
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Print);
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err("trailing tokens".to_string());
                }
                output.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(output)
}
