fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instructions = Vec::new();
    let mut next_index = 0;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !valid_label(name) {
                return Err(format!("line {line_number}: invalid label"));
            }
            if labels.insert(name, next_index).is_some() {
                return Err(format!("line {line_number}: duplicate label"));
            }
        } else {
            instructions.push((line_number, line));
            next_index += 1;
        }
    }

    let mut ops = Vec::with_capacity(instructions.len());

    for (line_number, line) in instructions {
        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();
        let operand = tokens.next();

        let op = match mnemonic {
            "push" => {
                let value = operand
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?
                    .parse::<i64>()
                    .map_err(|_| format!("line {line_number}: invalid operand"))?;
                Op::Push(value)
            }
            "jmp" | "jz" => {
                let name =
                    operand.ok_or_else(|| format!("line {line_number}: missing operand"))?;
                if !valid_label(name) {
                    return Err(format!("line {line_number}: invalid operand"));
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
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        if operand.is_some() && !matches!(mnemonic, "push" | "jmp" | "jz") {
            return Err(format!("line {line_number}: trailing tokens"));
        }
        if tokens.next().is_some() {
            return Err(format!("line {line_number}: trailing tokens"));
        }

        ops.push(op);
    }

    Ok(ops)
}
