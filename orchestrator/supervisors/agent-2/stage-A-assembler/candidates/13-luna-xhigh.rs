pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut lines = Vec::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        let code = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];
            if !is_label_name(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            if tokens[0].ends_with(':') {
                return Err("invalid label".to_string());
            }
            lines.push(tokens);
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for tokens in lines {
        match tokens[0] {
            "push" => {
                expect_arity(&tokens, 2)?;
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;
                ops.push(Op::Push(value));
            }
            "pop" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Pop);
            }
            "add" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Add);
            }
            "sub" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Sub);
            }
            "mul" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Mul);
            }
            "div" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Div);
            }
            "mod" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Mod);
            }
            "neg" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Neg);
            }
            "dup" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Dup);
            }
            "swap" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Swap);
            }
            "jmp" => {
                expect_arity(&tokens, 2)?;
                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?;
                ops.push(Op::Jmp(target));
            }
            "jz" => {
                expect_arity(&tokens, 2)?;
                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?;
                ops.push(Op::Jz(target));
            }
            "print" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Print);
            }
            "halt" => {
                expect_arity(&tokens, 1)?;
                ops.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(ops)
}

fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn expect_arity(tokens: &[&str], expected: usize) -> Result<(), String> {
    if tokens.len() < expected {
        Err("missing operand".to_string())
    } else if tokens.len() > expected {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}
