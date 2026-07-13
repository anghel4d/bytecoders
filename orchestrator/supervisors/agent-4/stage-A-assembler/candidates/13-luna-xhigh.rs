fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let lines: Vec<Vec<&str>> = src
        .lines()
        .filter_map(|line| {
            let code = line.split(';').next().unwrap_or("");
            let tokens: Vec<&str> = code.split_whitespace().collect();
            (!tokens.is_empty()).then_some(tokens)
        })
        .collect();

    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for tokens in &lines {
        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("invalid label".to_string());
            }

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

    for tokens in &lines {
        if tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                if tokens.len() != 2 {
                    return Err("invalid push".to_string());
                }

                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid push".to_string())?;
                Op::Push(value)
            }
            "pop" => {
                if tokens.len() != 1 {
                    return Err("invalid pop".to_string());
                }
                Op::Pop
            }
            "add" => {
                if tokens.len() != 1 {
                    return Err("invalid add".to_string());
                }
                Op::Add
            }
            "sub" => {
                if tokens.len() != 1 {
                    return Err("invalid sub".to_string());
                }
                Op::Sub
            }
            "mul" => {
                if tokens.len() != 1 {
                    return Err("invalid mul".to_string());
                }
                Op::Mul
            }
            "div" => {
                if tokens.len() != 1 {
                    return Err("invalid div".to_string());
                }
                Op::Div
            }
            "mod" => {
                if tokens.len() != 1 {
                    return Err("invalid mod".to_string());
                }
                Op::Mod
            }
            "neg" => {
                if tokens.len() != 1 {
                    return Err("invalid neg".to_string());
                }
                Op::Neg
            }
            "dup" => {
                if tokens.len() != 1 {
                    return Err("invalid dup".to_string());
                }
                Op::Dup
            }
            "swap" => {
                if tokens.len() != 1 {
                    return Err("invalid swap".to_string());
                }
                Op::Swap
            }
            "jmp" | "jz" => {
                if tokens.len() != 2 || !is_label_name(tokens[1]) {
                    return Err("invalid jump".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                if tokens[0] == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => {
                if tokens.len() != 1 {
                    return Err("invalid print".to_string());
                }
                Op::Print
            }
            "halt" => {
                if tokens.len() != 1 {
                    return Err("invalid halt".to_string());
                }
                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        ops.push(op);
    }

    Ok(ops)
}
