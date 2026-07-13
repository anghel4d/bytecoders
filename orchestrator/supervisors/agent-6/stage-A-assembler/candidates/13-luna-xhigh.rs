fn label_name_valid(name: &str) -> bool {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(b) if b.is_ascii_alphabetic() || b == b'_' => {}
        _ => return false,
    }

    bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn decimal_i64(token: &str) -> Option<i64> {
    let bytes = token.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len() || !bytes[start..].iter().all(|b| b.is_ascii_digit()) {
        return None;
    }

    token.parse::<i64>().ok()
}

fn resolve_label(
    name: &str,
    line: usize,
    labels: &std::collections::HashMap<String, usize>,
) -> Result<usize, String> {
    if !label_name_valid(name) {
        return Err(format!("line {}: invalid label", line));
    }

    labels
        .get(name)
        .copied()
        .ok_or_else(|| format!("line {}: undefined label", line))
}

fn assemble_instruction(
    tokens: &[&str],
    line: usize,
    labels: &std::collections::HashMap<String, usize>,
) -> Result<Op, String> {
    match tokens[0] {
        "push" => {
            if tokens.len() != 2 {
                return Err(format!("line {}: invalid operands", line));
            }

            let value = decimal_i64(tokens[1])
                .ok_or_else(|| format!("line {}: invalid integer", line))?;

            Ok(Op::Push(value))
        }
        "pop" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Pop)
        }
        "add" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Add)
        }
        "sub" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Sub)
        }
        "mul" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Mul)
        }
        "div" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Div)
        }
        "mod" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Mod)
        }
        "neg" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Neg)
        }
        "dup" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Dup)
        }
        "swap" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Swap)
        }
        "jmp" => {
            if tokens.len() != 2 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Jmp(resolve_label(tokens[1], line, labels)?))
        }
        "jz" => {
            if tokens.len() != 2 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Jz(resolve_label(tokens[1], line, labels)?))
        }
        "print" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Print)
        }
        "halt" => {
            if tokens.len() != 1 {
                return Err(format!("line {}: invalid operands", line));
            }

            Ok(Op::Halt)
        }
        _ => Err(format!("line {}: unknown mnemonic", line)),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line = line_index + 1;
        let code = raw_line.split(';').next().unwrap_or("").trim();

        if code.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.len() == 1 {
            if let Some(name) = tokens[0].strip_suffix(':') {
                if !label_name_valid(name) {
                    return Err(format!("line {}: invalid label", line));
                }
                if labels.contains_key(name) {
                    return Err(format!("line {}: duplicate label", line));
                }

                labels.insert(name.to_owned(), instruction_count);
                continue;
            }
        }

        if tokens[0].ends_with(':') {
            return Err(format!("line {}: label must be alone", line));
        }

        instruction_count += 1;
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line = line_index + 1;
        let code = raw_line.split(';').next().unwrap_or("").trim();

        if code.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        ops.push(assemble_instruction(&tokens, line, &labels)?);
    }

    Ok(ops)
}
