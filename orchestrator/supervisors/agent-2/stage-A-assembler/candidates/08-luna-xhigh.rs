fn source_tokens(line: &str) -> Vec<&str> {
    line.split(';')
        .next()
        .unwrap_or("")
        .split_whitespace()
        .collect()
}

fn valid_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn label_declaration<'a>(tokens: &[&'a str]) -> Result<Option<&'a str>, String> {
    if tokens.is_empty() || !tokens[0].ends_with(':') {
        return Ok(None);
    }

    if tokens.len() != 1 {
        return Err("trailing tokens".to_string());
    }

    let name = &tokens[0][..tokens[0].len() - 1];
    if !valid_label_name(name) {
        return Err("invalid label".to_string());
    }

    Ok(Some(name))
}

fn parse_integer(token: &str) -> Result<i64, String> {
    let bytes = token.as_bytes();
    let start = usize::from(bytes.first() == Some(&b'-'));

    if start == bytes.len() || !bytes[start..].iter().all(u8::is_ascii_digit) {
        return Err("invalid operand".to_string());
    }

    token
        .parse::<i64>()
        .map_err(|_| "invalid operand".to_string())
}

fn check_arity(tokens: &[&str], expected: usize) -> Result<(), String> {
    if tokens.len() < expected {
        Err("missing operand".to_string())
    } else if tokens.len() > expected {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}

fn validate_instruction(tokens: &[&str]) -> Result<(), String> {
    match tokens[0] {
        "push" => {
            check_arity(tokens, 2)?;
            parse_integer(tokens[1]).map(|_| ())
        }
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap" | "print"
        | "halt" => check_arity(tokens, 1),
        "jmp" | "jz" => {
            check_arity(tokens, 2)?;
            if valid_label_name(tokens[1]) {
                Ok(())
            } else {
                Err("invalid operand".to_string())
            }
        }
        _ => Err("unknown mnemonic".to_string()),
    }
}

fn emit_instruction(
    tokens: &[&str],
    labels: &std::collections::HashMap<String, usize>,
) -> Result<Op, String> {
    match tokens[0] {
        "push" => Ok(Op::Push(parse_integer(tokens[1])?)),
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

    for line in src.lines() {
        let tokens = source_tokens(line);
        if tokens.is_empty() {
            continue;
        }

        if let Some(name) = label_declaration(&tokens)? {
            if labels
                .insert(name.to_string(), instruction_count)
                .is_some()
            {
                return Err("duplicate label".to_string());
            }
        } else {
            validate_instruction(&tokens)?;
            instruction_count += 1;
        }
    }

    let mut code = Vec::with_capacity(instruction_count);

    for line in src.lines() {
        let tokens = source_tokens(line);
        if tokens.is_empty() {
            continue;
        }

        if label_declaration(&tokens)?.is_some() {
            continue;
        }

        code.push(emit_instruction(&tokens, &labels)?);
    }

    Ok(code)
}
