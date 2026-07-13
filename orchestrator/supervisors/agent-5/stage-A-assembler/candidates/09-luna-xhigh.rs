fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_integer(token: &str) -> Result<i64, String> {
    let digits = if let Some(rest) = token.strip_prefix('-') {
        rest
    } else {
        token
    };

    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err("invalid operand".to_string());
    }

    token
        .parse::<i64>()
        .map_err(|_| "invalid operand".to_string())
}

fn resolve_label(
    token: &str,
    labels: &std::collections::HashMap<String, usize>,
) -> Result<usize, String> {
    if !is_label_name(token) {
        return Err("invalid operand".to_string());
    }

    labels
        .get(token)
        .copied()
        .ok_or_else(|| "undefined label".to_string())
}

fn assemble_instruction(
    tokens: &[&str],
    labels: &std::collections::HashMap<String, usize>,
) -> Result<Op, String> {
    let mnemonic = tokens[0];

    match mnemonic {
        "push" => {
            if tokens.len() < 2 {
                return Err("missing operand".to_string());
            }
            if tokens.len() > 2 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Push(parse_integer(tokens[1])?))
        }
        "pop" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Pop)
        }
        "add" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Add)
        }
        "sub" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Sub)
        }
        "mul" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Mul)
        }
        "div" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Div)
        }
        "mod" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Mod)
        }
        "neg" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Neg)
        }
        "dup" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Dup)
        }
        "swap" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Swap)
        }
        "jmp" => {
            if tokens.len() < 2 {
                return Err("missing operand".to_string());
            }
            if tokens.len() > 2 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Jmp(resolve_label(tokens[1], labels)?))
        }
        "jz" => {
            if tokens.len() < 2 {
                return Err("missing operand".to_string());
            }
            if tokens.len() > 2 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Jz(resolve_label(tokens[1], labels)?))
        }
        "print" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Print)
        }
        "halt" => {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }
            Ok(Op::Halt)
        }
        _ => Err("unknown mnemonic".to_string()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (line_number, raw_line) in src.lines().enumerate() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let first = tokens.next().unwrap();

        if first.ends_with(':') {
            if tokens.next().is_some() {
                return Err(format!("line {}: trailing tokens", line_number + 1));
            }

            let name = &first[..first.len() - 1];
            if !is_label_name(name) {
                return Err(format!("line {}: invalid label", line_number + 1));
            }
            if labels
                .insert(name.to_string(), instruction_count)
                .is_some()
            {
                return Err(format!("line {}: duplicate label", line_number + 1));
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut program = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens[0].ends_with(':') {
            continue;
        }

        program.push(assemble_instruction(&tokens, &labels)?);
    }

    Ok(program)
}
