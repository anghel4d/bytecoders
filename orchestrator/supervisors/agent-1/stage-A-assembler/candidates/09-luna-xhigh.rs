pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let is_label = |name: &str| -> bool {
        let mut chars = name.chars();
        match chars.next() {
            Some(c) if c == '_' || c.is_ascii_alphabetic() => {}
            _ => return false,
        }
        chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
    };

    let parse_int = |text: &str| -> Option<i64> {
        let bytes = text.as_bytes();
        let digits = if bytes.first() == Some(&b'-') {
            &bytes[1..]
        } else {
            bytes
        };

        if digits.is_empty() || !digits.iter().all(|b| b.is_ascii_digit()) {
            return None;
        }

        text.parse::<i64>().ok()
    };

    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let code = raw_line.split(';').next().unwrap_or("").trim();

        if code.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !is_label(name) {
                return Err(format!("line {}: invalid label", line_number));
            }

            if labels
                .insert(name.to_string(), instruction_count)
                .is_some()
            {
                return Err(format!("line {}: duplicate label", line_number));
            }

            continue;
        }

        match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                if parse_int(tokens[1]).is_none() {
                    return Err(format!("line {}: invalid integer", line_number));
                }
            }
            "jmp" | "jz" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line_number));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
                if !is_label(tokens[1]) {
                    return Err(format!("line {}: invalid label", line_number));
                }
            }
            "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
            | "print" | "halt" => {
                if tokens.len() > 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }
            }
            _ => {
                return Err(format!("line {}: unknown mnemonic", line_number));
            }
        }

        instruction_count += 1;
    }

    let mut program = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let code = raw_line.split(';').next().unwrap_or("").trim();

        if code.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => match parse_int(tokens[1]) {
                Some(value) => Op::Push(value),
                None => return Err(format!("line {}: invalid integer", line_number)),
            },
            "pop" => Op::Pop,
            "add" => Op::Add,
            "sub" => Op::Sub,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "neg" => Op::Neg,
            "dup" => Op::Dup,
            "swap" => Op::Swap,
            "jmp" => match labels.get(tokens[1]) {
                Some(&target) => Op::Jmp(target),
                None => return Err(format!("line {}: undefined label", line_number)),
            },
            "jz" => match labels.get(tokens[1]) {
                Some(&target) => Op::Jz(target),
                None => return Err(format!("line {}: undefined label", line_number)),
            },
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => return Err(format!("line {}: unknown mnemonic", line_number)),
        };

        program.push(op);
    }

    Ok(program)
}
