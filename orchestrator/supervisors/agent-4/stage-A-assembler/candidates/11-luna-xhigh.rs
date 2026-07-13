fn assemble_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn assemble_arity(tokens: &[&str], operands: usize) -> Result<(), String> {
    let actual = tokens.len() - 1;

    if actual < operands {
        Err("missing operand".to_string())
    } else if actual > operands {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instructions = Vec::new();

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !assemble_label_name(name) {
                return Err("invalid label".to_string());
            }

            if labels.insert(name.to_string(), instructions.len()).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instructions.push(tokens);
        }
    }

    let mut ops = Vec::with_capacity(instructions.len());

    for tokens in instructions {
        let op = match tokens[0] {
            "push" => {
                assemble_arity(&tokens, 1)?;
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;
                Op::Push(value)
            }
            "pop" => {
                assemble_arity(&tokens, 0)?;
                Op::Pop
            }
            "add" => {
                assemble_arity(&tokens, 0)?;
                Op::Add
            }
            "sub" => {
                assemble_arity(&tokens, 0)?;
                Op::Sub
            }
            "mul" => {
                assemble_arity(&tokens, 0)?;
                Op::Mul
            }
            "div" => {
                assemble_arity(&tokens, 0)?;
                Op::Div
            }
            "mod" => {
                assemble_arity(&tokens, 0)?;
                Op::Mod
            }
            "neg" => {
                assemble_arity(&tokens, 0)?;
                Op::Neg
            }
            "dup" => {
                assemble_arity(&tokens, 0)?;
                Op::Dup
            }
            "swap" => {
                assemble_arity(&tokens, 0)?;
                Op::Swap
            }
            "jmp" => {
                assemble_arity(&tokens, 1)?;

                if !assemble_label_name(tokens[1]) {
                    return Err("invalid operand".to_string());
                }

                let target = match labels.get(tokens[1]) {
                    Some(&target) => target,
                    None => return Err("undefined label".to_string()),
                };

                Op::Jmp(target)
            }
            "jz" => {
                assemble_arity(&tokens, 1)?;

                if !assemble_label_name(tokens[1]) {
                    return Err("invalid operand".to_string());
                }

                let target = match labels.get(tokens[1]) {
                    Some(&target) => target,
                    None => return Err("undefined label".to_string()),
                };

                Op::Jz(target)
            }
            "print" => {
                assemble_arity(&tokens, 0)?;
                Op::Print
            }
            "halt" => {
                assemble_arity(&tokens, 0)?;
                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        ops.push(op);
    }

    Ok(ops)
}
