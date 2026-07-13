fn is_valid_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn expect_arity(tokens: &[&str], arity: usize) -> Result<(), String> {
    if tokens.len() < arity {
        Err("missing operand".to_string())
    } else if tokens.len() > arity {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}

fn resolve_label(
    labels: &std::collections::HashMap<String, usize>,
    name: &str,
) -> Result<usize, String> {
    if !is_valid_label(name) {
        return Err("invalid label".to_string());
    }

    labels
        .get(name)
        .copied()
        .ok_or_else(|| "undefined label".to_string())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instructions = Vec::new();

    for line in src.lines() {
        let code = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens.len() == 1 {
            if let Some(name) = tokens[0].strip_suffix(':') {
                if !is_valid_label(name) {
                    return Err("invalid label".to_string());
                }

                if labels
                    .insert(name.to_string(), instructions.len())
                    .is_some()
                {
                    return Err("duplicate label".to_string());
                }

                continue;
            }
        }

        instructions.push(tokens);
    }

    let mut output = Vec::with_capacity(instructions.len());

    for tokens in instructions {
        match tokens[0] {
            "push" => {
                expect_arity(&tokens, 2)?;
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;
                output.push(Op::Push(value));
            }
            "pop" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Pop);
            }
            "add" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Add);
            }
            "sub" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Sub);
            }
            "mul" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Mul);
            }
            "div" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Div);
            }
            "mod" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Mod);
            }
            "neg" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Neg);
            }
            "dup" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Dup);
            }
            "swap" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Swap);
            }
            "jmp" => {
                expect_arity(&tokens, 2)?;
                let target = resolve_label(&labels, tokens[1])?;
                output.push(Op::Jmp(target));
            }
            "jz" => {
                expect_arity(&tokens, 2)?;
                let target = resolve_label(&labels, tokens[1])?;
                output.push(Op::Jz(target));
            }
            "print" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Print);
            }
            "halt" => {
                expect_arity(&tokens, 1)?;
                output.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(output)
}
