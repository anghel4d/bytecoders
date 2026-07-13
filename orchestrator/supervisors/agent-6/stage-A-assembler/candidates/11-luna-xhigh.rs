pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let is_label_name = |name: &str| {
        let mut chars = name.chars();
        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
            _ => return false,
        }
        chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
    };

    let check_arity = |tokens: &[&str], expected: usize| -> Result<(), String> {
        if tokens.len() < expected {
            Err("missing operand".to_string())
        } else if tokens.len() > expected {
            Err("trailing tokens".to_string())
        } else {
            Ok(())
        }
    };

    let mut labels = std::collections::HashMap::new();
    let mut lines = Vec::new();

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];
            if !is_label_name(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, lines.len()).is_some() {
                return Err("duplicate label".to_string());
            }
            continue;
        }

        if tokens[0].ends_with(':') {
            return Err("invalid label".to_string());
        }

        lines.push(tokens);
    }

    let mut ops = Vec::with_capacity(lines.len());

    for tokens in lines {
        match tokens[0] {
            "push" => {
                check_arity(&tokens, 2)?;
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid integer".to_string())?;
                ops.push(Op::Push(value));
            }
            "pop" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Pop);
            }
            "add" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Add);
            }
            "sub" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Sub);
            }
            "mul" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Mul);
            }
            "div" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Div);
            }
            "mod" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Mod);
            }
            "neg" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Neg);
            }
            "dup" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Dup);
            }
            "swap" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Swap);
            }
            "jmp" => {
                check_arity(&tokens, 2)?;
                if !is_label_name(tokens[1]) {
                    return Err("invalid operand".to_string());
                }
                let target = match labels.get(tokens[1]) {
                    Some(&target) => target,
                    None => return Err("undefined label".to_string()),
                };
                ops.push(Op::Jmp(target));
            }
            "jz" => {
                check_arity(&tokens, 2)?;
                if !is_label_name(tokens[1]) {
                    return Err("invalid operand".to_string());
                }
                let target = match labels.get(tokens[1]) {
                    Some(&target) => target,
                    None => return Err("undefined label".to_string()),
                };
                ops.push(Op::Jz(target));
            }
            "print" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Print);
            }
            "halt" => {
                check_arity(&tokens, 1)?;
                ops.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(ops)
}
