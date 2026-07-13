fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn expect_tokens(tokens: &[&str], expected: usize) -> Result<(), String> {
    if tokens.len() < expected {
        Err("missing operand".to_string())
    } else if tokens.len() > expected {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instructions = Vec::new();

    for line in src.lines() {
        let line = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !is_label_name(name) {
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
                expect_tokens(&tokens, 2)?;
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid integer".to_string())?;
                output.push(Op::Push(value));
            }
            "pop" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Pop);
            }
            "add" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Add);
            }
            "sub" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Sub);
            }
            "mul" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Mul);
            }
            "div" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Div);
            }
            "mod" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Mod);
            }
            "neg" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Neg);
            }
            "dup" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Dup);
            }
            "swap" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Swap);
            }
            "jmp" => {
                expect_tokens(&tokens, 2)?;

                if !is_label_name(tokens[1]) {
                    return Err("invalid label".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                output.push(Op::Jmp(target));
            }
            "jz" => {
                expect_tokens(&tokens, 2)?;

                if !is_label_name(tokens[1]) {
                    return Err("invalid label".to_string());
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                output.push(Op::Jz(target));
            }
            "print" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Print);
            }
            "halt" => {
                expect_tokens(&tokens, 1)?;
                output.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(output)
}
