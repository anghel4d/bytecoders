pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut lines = Vec::new();

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if !tokens.is_empty() {
            lines.push(tokens);
        }
    }

    let mut labels = std::collections::HashMap::new();
    let mut instruction_index = 0usize;

    for tokens in &lines {
        if let Some(name) = tokens[0].strip_suffix(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            if !is_assembly_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, instruction_index).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_index += 1;
        }
    }

    let mut code = Vec::with_capacity(instruction_index);

    for tokens in &lines {
        if tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                Op::Push(parse_assembly_int(tokens[1])?)
            }
            "pop" => {
                require_no_operands(tokens)?;
                Op::Pop
            }
            "add" => {
                require_no_operands(tokens)?;
                Op::Add
            }
            "sub" => {
                require_no_operands(tokens)?;
                Op::Sub
            }
            "mul" => {
                require_no_operands(tokens)?;
                Op::Mul
            }
            "div" => {
                require_no_operands(tokens)?;
                Op::Div
            }
            "mod" => {
                require_no_operands(tokens)?;
                Op::Mod
            }
            "neg" => {
                require_no_operands(tokens)?;
                Op::Neg
            }
            "dup" => {
                require_no_operands(tokens)?;
                Op::Dup
            }
            "swap" => {
                require_no_operands(tokens)?;
                Op::Swap
            }
            "jmp" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                Op::Jmp(resolve_assembly_label(&labels, tokens[1])?)
            }
            "jz" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                Op::Jz(resolve_assembly_label(&labels, tokens[1])?)
            }
            "print" => {
                require_no_operands(tokens)?;
                Op::Print
            }
            "halt" => {
                require_no_operands(tokens)?;
                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        code.push(op);
    }

    Ok(code)
}

fn is_assembly_label(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
        return false;
    }
    bytes[1..]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
}

fn parse_assembly_int(value: &str) -> Result<i64, String> {
    let bytes = value.as_bytes();
    let digits = if bytes.first() == Some(&b'-') {
        &bytes[1..]
    } else {
        bytes
    };

    if digits.is_empty() || !digits.iter().all(|byte| byte.is_ascii_digit()) {
        return Err("invalid operand".to_string());
    }

    value
        .parse::<i64>()
        .map_err(|_| "invalid operand".to_string())
}

fn resolve_assembly_label<'a>(
    labels: &std::collections::HashMap<&'a str, usize>,
    name: &str,
) -> Result<usize, String> {
    if !is_assembly_label(name) {
        return Err("invalid operand".to_string());
    }

    labels
        .get(name)
        .copied()
        .ok_or_else(|| "undefined label".to_string())
}

fn require_no_operands(tokens: &[&str]) -> Result<(), String> {
    if tokens.len() != 1 {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}
