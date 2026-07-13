fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn ensure_no_tokens<'a, I>(tokens: &mut I) -> Result<(), String>
where
    I: Iterator<Item = &'a str>,
{
    if tokens.next().is_some() {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<&str, usize>::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let first = tokens.next().unwrap();

        if let Some(name) = first.strip_suffix(':') {
            ensure_no_tokens(&mut tokens)?;
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

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        if mnemonic.ends_with(':') {
            continue;
        }

        match mnemonic {
            "push" => {
                let operand = tokens.next().ok_or_else(|| "missing operand".to_string())?;
                ensure_no_tokens(&mut tokens)?;
                let value = operand
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;
                ops.push(Op::Push(value));
            }
            "pop" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Pop);
            }
            "add" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Add);
            }
            "sub" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Sub);
            }
            "mul" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Mul);
            }
            "div" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Div);
            }
            "mod" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Mod);
            }
            "neg" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Neg);
            }
            "dup" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Dup);
            }
            "swap" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Swap);
            }
            "jmp" | "jz" => {
                let operand = tokens.next().ok_or_else(|| "missing operand".to_string())?;
                ensure_no_tokens(&mut tokens)?;

                if !valid_label(operand) {
                    return Err("invalid operand".to_string());
                }

                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                if mnemonic == "jmp" {
                    ops.push(Op::Jmp(target));
                } else {
                    ops.push(Op::Jz(target));
                }
            }
            "print" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Print);
            }
            "halt" => {
                ensure_no_tokens(&mut tokens)?;
                ops.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(ops)
}
