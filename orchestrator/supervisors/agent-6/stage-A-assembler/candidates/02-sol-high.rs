fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();

    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for (line_number, raw_line) in src.lines().enumerate() {
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let first = tokens.next().unwrap();

        if tokens.next().is_none() {
            if let Some(name) = first.strip_suffix(':') {
                if !valid_label(name) {
                    return Err(format!("line {}: invalid label", line_number + 1));
                }
                if labels
                    .insert(name.to_owned(), instruction_count)
                    .is_some()
                {
                    return Err(format!("line {}: duplicate label", line_number + 1));
                }
                continue;
            }
        }

        instruction_count += 1;
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_number, raw_line) in src.lines().enumerate() {
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        if tokens.next().is_none() && mnemonic.ends_with(':') {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();
        let line_number = line_number + 1;

        let op = match mnemonic {
            "push" => {
                let operand = tokens
                    .next()
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?;
                let value = operand
                    .parse::<i64>()
                    .map_err(|_| format!("line {line_number}: invalid integer"))?;
                Op::Push(value)
            }
            "pop" => Op::Pop,
            "add" => Op::Add,
            "sub" => Op::Sub,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "neg" => Op::Neg,
            "dup" => Op::Dup,
            "swap" => Op::Swap,
            "jmp" | "jz" => {
                let name = tokens
                    .next()
                    .ok_or_else(|| format!("line {line_number}: missing label"))?;
                if !valid_label(name) {
                    return Err(format!("line {line_number}: invalid label"));
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
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        if tokens.next().is_some() {
            return Err(format!("line {line_number}: trailing tokens"));
        }

        ops.push(op);
    }

    Ok(ops)
}
