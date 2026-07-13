fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();

    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        let line = line.split_once(';').map_or(line, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        if let Some(name) = mnemonic.strip_suffix(':') {
            if !valid_label(name) {
                return Err("invalid label".into());
            }
            if tokens.next().is_some() {
                return Err("trailing tokens".into());
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".into());
            }
            continue;
        }

        match mnemonic {
            "push" => {
                let operand = tokens.next().ok_or_else(|| "missing operand".to_string())?;
                operand
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;
            }
            "jmp" | "jz" => {
                let operand = tokens.next().ok_or_else(|| "missing operand".to_string())?;
                if !valid_label(operand) {
                    return Err("invalid operand".into());
                }
            }
            "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
            | "print" | "halt" => {}
            _ => return Err("unknown mnemonic".into()),
        }

        if tokens.next().is_some() {
            return Err("trailing tokens".into());
        }
        instruction_count += 1;
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for line in src.lines() {
        let line = line.split_once(';').map_or(line, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        if mnemonic.ends_with(':') {
            continue;
        }

        let op = match mnemonic {
            "push" => Op::Push(tokens.next().unwrap().parse::<i64>().unwrap()),
            "pop" => Op::Pop,
            "add" => Op::Add,
            "sub" => Op::Sub,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "neg" => Op::Neg,
            "dup" => Op::Dup,
            "swap" => Op::Swap,
            "jmp" => {
                let label = tokens.next().unwrap();
                let target = labels
                    .get(label)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jmp(target)
            }
            "jz" => {
                let label = tokens.next().unwrap();
                let target = labels
                    .get(label)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jz(target)
            }
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => unreachable!(),
        };

        ops.push(op);
    }

    Ok(ops)
}
