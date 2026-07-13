fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_'))
}

fn parse_integer(text: &str) -> Result<i64, ()> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(());
    }
    text.parse().map_err(|_| ())
}

fn check_instruction(tokens: &[&str]) -> Result<(), String> {
    let arity = match tokens[0] {
        "push" | "jmp" | "jz" => 2,
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
        | "print" | "halt" => 1,
        mnemonic => return Err(format!("unknown mnemonic: {mnemonic}")),
    };

    if tokens.len() < arity {
        return Err("missing operand".to_string());
    }
    if tokens.len() > arity {
        return Err("trailing tokens".to_string());
    }

    match tokens[0] {
        "push" if parse_integer(tokens[1]).is_err() => Err("invalid integer".to_string()),
        "jmp" | "jz" if !valid_label(tokens[1]) => Err("invalid label".to_string()),
        _ => Ok(()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 {
            if let Some(name) = tokens[0].strip_suffix(':') {
                if !valid_label(name) {
                    return Err(format!("line {}: invalid label", line_index + 1));
                }
                if labels.insert(name.to_string(), instruction_count).is_some() {
                    return Err(format!("line {}: duplicate label", line_index + 1));
                }
                continue;
            }
        }

        check_instruction(&tokens)
            .map_err(|message| format!("line {}: {message}", line_index + 1))?;
        instruction_count += 1;
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
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
                    .map_err(|_| format!("line {}: invalid integer", line_index + 1))?,
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
                    .ok_or_else(|| format!("line {}: undefined label", line_index + 1))?,
            ),
            "jz" => Op::Jz(
                labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| format!("line {}: undefined label", line_index + 1))?,
            ),
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => unreachable!(),
        };

        ops.push(op);
    }

    Ok(ops)
}
