fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn label_definition(line: &str) -> Result<Option<&str>, &'static str> {
    let mut tokens = line.split_whitespace();
    let token = tokens.next().unwrap();

    if tokens.next().is_some() || !token.ends_with(':') {
        return Ok(None);
    }

    let name = &token[..token.len() - 1];
    if !is_label_name(name) {
        return Err("invalid label");
    }

    Ok(Some(name))
}

fn parse_integer(token: &str) -> Result<i64, &'static str> {
    let digits = token.strip_prefix('-').unwrap_or(token);

    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("invalid operand");
    }

    token.parse().map_err(|_| "invalid operand")
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_index = 0;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line = raw_line
            .split_once(';')
            .map_or(raw_line, |(code, _)| code)
            .trim();

        if line.is_empty() {
            continue;
        }

        match label_definition(line) {
            Ok(Some(name)) => {
                if labels.insert(name, instruction_index).is_some() {
                    return Err(format!("line {}: duplicate label", line_index + 1));
                }
            }
            Ok(None) => instruction_index += 1,
            Err(message) => return Err(format!("line {}: {}", line_index + 1, message)),
        }
    }

    let mut ops = Vec::with_capacity(instruction_index);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line = raw_line
            .split_once(';')
            .map_or(raw_line, |(code, _)| code)
            .trim();

        if line.is_empty() {
            continue;
        }

        match label_definition(line) {
            Ok(Some(_)) => continue,
            Ok(None) => {}
            Err(message) => return Err(format!("line {}: {}", line_index + 1, message)),
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();
        let operand = tokens.next();
        let trailing = tokens.next();

        let op = match mnemonic {
            "push" => {
                let value = operand.ok_or_else(|| {
                    format!("line {}: missing operand", line_index + 1)
                })?;
                if trailing.is_some() {
                    return Err(format!("line {}: trailing tokens", line_index + 1));
                }
                Op::Push(
                    parse_integer(value)
                        .map_err(|message| format!("line {}: {}", line_index + 1, message))?,
                )
            }
            "jmp" | "jz" => {
                let name = operand.ok_or_else(|| {
                    format!("line {}: missing operand", line_index + 1)
                })?;
                if trailing.is_some() {
                    return Err(format!("line {}: trailing tokens", line_index + 1));
                }
                if !is_label_name(name) {
                    return Err(format!("line {}: invalid operand", line_index + 1));
                }
                let target = labels.get(name).copied().ok_or_else(|| {
                    format!("line {}: undefined label", line_index + 1)
                })?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup"
            | "swap" | "print" | "halt" => {
                if operand.is_some() {
                    return Err(format!("line {}: trailing tokens", line_index + 1));
                }
                match mnemonic {
                    "pop" => Op::Pop,
                    "add" => Op::Add,
                    "sub" => Op::Sub,
                    "mul" => Op::Mul,
                    "div" => Op::Div,
                    "mod" => Op::Mod,
                    "neg" => Op::Neg,
                    "dup" => Op::Dup,
                    "swap" => Op::Swap,
                    "print" => Op::Print,
                    "halt" => Op::Halt,
                    _ => unreachable!(),
                }
            }
            _ => return Err(format!("line {}: unknown mnemonic", line_index + 1)),
        };

        ops.push(op);
    }

    Ok(ops)
}
