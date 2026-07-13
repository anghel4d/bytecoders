pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let code = raw_line.split(';').next().unwrap_or("").trim();
        if code.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = code.split_whitespace().collect();
        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];
            if !is_assemble_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name.to_string(), instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            if tokens.first().is_some_and(|token| token.ends_with(':')) {
                return Err("invalid label".to_string());
            }
            instruction_count += 1;
        }
    }

    let mut output = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let code = raw_line.split(';').next().unwrap_or("").trim();
        if code.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = code.split_whitespace().collect();
        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        output.push(parse_assemble_instruction(&tokens, &labels)?);
    }

    Ok(output)
}

fn is_assemble_label(name: &str) -> bool {
    let bytes = name.as_bytes();

    if bytes.is_empty()
        || !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_')
    {
        return false;
    }

    bytes[1..]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
}

fn assemble_jump_target(
    name: &str,
    labels: &std::collections::HashMap<String, usize>,
) -> Result<usize, String> {
    if !is_assemble_label(name) {
        return Err("invalid label".to_string());
    }

    labels
        .get(name)
        .copied()
        .ok_or_else(|| "undefined label".to_string())
}

fn parse_assemble_instruction(
    tokens: &[&str],
    labels: &std::collections::HashMap<String, usize>,
) -> Result<Op, String> {
    if tokens.is_empty() {
        return Err("missing instruction".to_string());
    }

    match tokens[0] {
        "push" => {
            if tokens.len() != 2 {
                return Err("invalid push operand".to_string());
            }

            tokens[1]
                .parse::<i64>()
                .map(Op::Push)
                .map_err(|_| "invalid integer".to_string())
        }
        "pop" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Pop)
            }
        }
        "add" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Add)
            }
        }
        "sub" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Sub)
            }
        }
        "mul" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Mul)
            }
        }
        "div" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Div)
            }
        }
        "mod" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Mod)
            }
        }
        "neg" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Neg)
            }
        }
        "dup" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Dup)
            }
        }
        "swap" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Swap)
            }
        }
        "jmp" => {
            if tokens.len() != 2 {
                return Err("invalid jmp operand".to_string());
            }

            Ok(Op::Jmp(assemble_jump_target(tokens[1], labels)?))
        }
        "jz" => {
            if tokens.len() != 2 {
                return Err("invalid jz operand".to_string());
            }

            Ok(Op::Jz(assemble_jump_target(tokens[1], labels)?))
        }
        "print" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Print)
            }
        }
        "halt" => {
            if tokens.len() != 1 {
                Err("trailing tokens".to_string())
            } else {
                Ok(Op::Halt)
            }
        }
        _ => Err("unknown mnemonic".to_string()),
    }
}
