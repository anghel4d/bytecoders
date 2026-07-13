fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn validate_instruction(tokens: &[&str]) -> Result<(), String> {
    let mnemonic = tokens.first().copied().ok_or_else(|| "missing instruction".to_string())?;

    match mnemonic {
        "push" => {
            if tokens.len() < 2 {
                return Err("missing operand".to_string());
            }
            if tokens.len() > 2 {
                return Err("trailing tokens".to_string());
            }
            tokens[1]
                .parse::<i64>()
                .map(|_| ())
                .map_err(|_| "invalid integer".to_string())
        }
        "jmp" | "jz" => {
            if tokens.len() < 2 {
                return Err("missing operand".to_string());
            }
            if tokens.len() > 2 {
                return Err("trailing tokens".to_string());
            }
            if valid_label(tokens[1]) {
                Ok(())
            } else {
                Err("invalid operand".to_string())
            }
        }
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
        | "print" | "halt" => {
            if tokens.len() == 1 {
                Ok(())
            } else {
                Err("trailing tokens".to_string())
            }
        }
        _ => Err("unknown mnemonic".to_string()),
    }
}

fn emit_instruction(
    tokens: &[&str],
    labels: &std::collections::HashMap<String, usize>,
) -> Result<Op, String> {
    let mnemonic = tokens.first().copied().ok_or_else(|| "missing instruction".to_string())?;

    match mnemonic {
        "push" => Ok(Op::Push(
            tokens[1]
                .parse::<i64>()
                .map_err(|_| "invalid integer".to_string())?,
        )),
        "pop" => Ok(Op::Pop),
        "add" => Ok(Op::Add),
        "sub" => Ok(Op::Sub),
        "mul" => Ok(Op::Mul),
        "div" => Ok(Op::Div),
        "mod" => Ok(Op::Mod),
        "neg" => Ok(Op::Neg),
        "dup" => Ok(Op::Dup),
        "swap" => Ok(Op::Swap),
        "jmp" => {
            let target = labels
                .get(tokens[1])
                .copied()
                .ok_or_else(|| "undefined label".to_string())?;
            Ok(Op::Jmp(target))
        }
        "jz" => {
            let target = labels
                .get(tokens[1])
                .copied()
                .ok_or_else(|| "undefined label".to_string())?;
            Ok(Op::Jz(target))
        }
        "print" => Ok(Op::Print),
        "halt" => Ok(Op::Halt),
        _ => Err("unknown mnemonic".to_string()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();

        if line.is_empty() {
            continue;
        }

        if line.ends_with(':') {
            let name = &line[..line.len() - 1];

            if !valid_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.contains_key(name) {
                return Err("duplicate label".to_string());
            }

            labels.insert(name.to_string(), instruction_count);
        } else {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            validate_instruction(&tokens)?;
            instruction_count += 1;
        }
    }

    let mut code = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();

        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        code.push(emit_instruction(&tokens, &labels)?);
    }

    Ok(code)
}
