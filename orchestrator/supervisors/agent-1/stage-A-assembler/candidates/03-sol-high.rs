fn assembly_code(line: &str) -> &str {
    line.split_once(';').map_or(line, |(code, _)| code).trim()
}

fn label_definition(line: &str) -> Option<&str> {
    let mut tokens = line.split_whitespace();
    let token = tokens.next()?;
    if tokens.next().is_none() {
        token.strip_suffix(':')
    } else {
        None
    }
}

fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn valid_integer(value: &str) -> bool {
    let digits = value.strip_prefix('-').unwrap_or(value);
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
}

fn assembly_error(line: usize, message: &str) -> String {
    format!("line {}: {}", line, message)
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (index, raw_line) in src.lines().enumerate() {
        let line_number = index + 1;
        let line = assembly_code(raw_line);

        if line.is_empty() {
            continue;
        }

        if let Some(name) = label_definition(line) {
            if !valid_label(name) {
                return Err(assembly_error(line_number, "invalid label"));
            }
            if labels.contains_key(name) {
                return Err(assembly_error(line_number, "duplicate label"));
            }
            labels.insert(name.to_owned(), instruction_count);
        } else {
            instruction_count = instruction_count
                .checked_add(1)
                .ok_or_else(|| assembly_error(line_number, "too many instructions"))?;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (index, raw_line) in src.lines().enumerate() {
        let line_number = index + 1;
        let line = assembly_code(raw_line);

        if line.is_empty() || label_definition(line).is_some() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = tokens[0];

        let op = match mnemonic {
            "push" => {
                if tokens.len() < 2 {
                    return Err(assembly_error(line_number, "missing operand"));
                }
                if tokens.len() > 2 {
                    return Err(assembly_error(line_number, "trailing tokens"));
                }
                if !valid_integer(tokens[1]) {
                    return Err(assembly_error(line_number, "invalid operand"));
                }
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| assembly_error(line_number, "invalid operand"))?;
                Op::Push(value)
            }
            "jmp" | "jz" => {
                if tokens.len() < 2 {
                    return Err(assembly_error(line_number, "missing operand"));
                }
                if tokens.len() > 2 {
                    return Err(assembly_error(line_number, "trailing tokens"));
                }
                if !valid_label(tokens[1]) {
                    return Err(assembly_error(line_number, "invalid operand"));
                }
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| assembly_error(line_number, "undefined label"))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
            | "print" | "halt" => {
                if tokens.len() > 1 {
                    return Err(assembly_error(line_number, "trailing tokens"));
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
            _ => return Err(assembly_error(line_number, "unknown mnemonic")),
        };

        ops.push(op);
    }

    Ok(ops)
}
