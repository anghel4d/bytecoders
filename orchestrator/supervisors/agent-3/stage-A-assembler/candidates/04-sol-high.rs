fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn operand<'a>(tokens: &'a [&str], line: usize) -> Result<&'a str, String> {
    match tokens {
        [_] => Err(format!("line {line}: missing operand")),
        [_, value] => Ok(value),
        _ => Err(format!("line {line}: trailing tokens")),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (index, raw_line) in src.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !is_label_name(name) {
                return Err(format!("line {line_number}: invalid label"));
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err(format!("line {line_number}: duplicate label"));
            }
        } else {
            instruction_count = instruction_count
                .checked_add(1)
                .ok_or_else(|| "too many instructions".to_string())?;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (index, raw_line) in src.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mnemonic = tokens[0];

        let op = match mnemonic {
            "push" => {
                let value = operand(&tokens, line_number)?
                    .parse::<i64>()
                    .map_err(|_| format!("line {line_number}: invalid integer"))?;
                Op::Push(value)
            }
            "jmp" | "jz" => {
                let name = operand(&tokens, line_number)?;
                if !is_label_name(name) {
                    return Err(format!("line {line_number}: invalid label operand"));
                }
                let target = labels
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup"
            | "swap" | "print" | "halt" => {
                if tokens.len() != 1 {
                    return Err(format!("line {line_number}: trailing tokens"));
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
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        ops.push(op);
    }

    Ok(ops)
}
