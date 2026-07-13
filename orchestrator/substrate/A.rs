fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !valid_label(name) {
                return Err(format!("line {line_number}: invalid label"));
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err(format!("line {line_number}: duplicate label"));
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        let op = match mnemonic {
            "push" => {
                let operand = tokens
                    .next()
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?;
                let value = operand
                    .parse::<i64>()
                    .map_err(|_| format!("line {line_number}: invalid operand"))?;
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
                let operand = tokens
                    .next()
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?;
                if !valid_label(operand) {
                    return Err(format!("line {line_number}: invalid operand"));
                }
                let target = labels
                    .get(operand)
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
