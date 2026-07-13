fn is_label_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    match bytes.next() {
        Some(b) if b.is_ascii_alphabetic() || b == b'_' => {}
        _ => return false,
    }
    bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn parse_integer(token: &str) -> Option<i64> {
    let digits = token.strip_prefix('-').unwrap_or(token);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    token.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 {
            if let Some(name) = tokens[0].strip_suffix(':') {
                if !is_label_name(name) {
                    return Err(format!("line {line_number}: invalid label"));
                }
                if labels.insert(name, instruction_count).is_some() {
                    return Err(format!("line {line_number}: duplicate label"));
                }
                continue;
            }
        }

        match tokens[0] {
            "push" => {
                if tokens.len() != 2 || parse_integer(tokens[1]).is_none() {
                    return Err(format!("line {line_number}: invalid push operand"));
                }
            }
            "jmp" | "jz" => {
                if tokens.len() != 2 || !is_label_name(tokens[1]) {
                    return Err(format!("line {line_number}: invalid label operand"));
                }
            }
            "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
            | "print" | "halt" => {
                if tokens.len() != 1 {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
            }
            mnemonic => {
                return Err(format!("line {line_number}: unknown mnemonic {mnemonic}"));
            }
        }

        instruction_count += 1;
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => Op::Push(
                parse_integer(tokens[1])
                    .ok_or_else(|| format!("line {line_number}: invalid push operand"))?,
            ),
            "pop" => Op::Pop,
            "add" => Op::Add,
            "sub" => Op::Sub,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "neg" => Op::Neg,
            "dup" => Op::Dup,
            "swap" => Op::Swap,
            "jmp" => Op::Jmp(
                labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?,
            ),
            "jz" => Op::Jz(
                labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?,
            ),
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => unreachable!(),
        };

        ops.push(op);
    }

    Ok(ops)
}
