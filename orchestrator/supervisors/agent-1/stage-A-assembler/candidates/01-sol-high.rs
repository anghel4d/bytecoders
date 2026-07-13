fn valid_label(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'A'..=b'Z' | b'a'..=b'z' | b'_'))
        && bytes.all(|byte| matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_'))
}

fn parse_integer(token: &str) -> Option<i64> {
    let digits = token
        .strip_prefix('-')
        .or_else(|| token.strip_prefix('+'))
        .unwrap_or(token);

    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    token.parse().ok()
}

fn assembly_error(line: usize, message: &str) -> String {
    format!("line {}: {}", line, message)
}

fn validate_instruction(tokens: &[&str], line: usize) -> Result<(), String> {
    let operands = match tokens[0] {
        "push" | "jmp" | "jz" => 1,
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
        | "print" | "halt" => 0,
        _ => return Err(assembly_error(line, "unknown mnemonic")),
    };

    if tokens.len() < operands + 1 {
        return Err(assembly_error(line, "missing operand"));
    }
    if tokens.len() > operands + 1 {
        return Err(assembly_error(line, "trailing tokens"));
    }

    match tokens[0] {
        "push" if parse_integer(tokens[1]).is_none() => {
            Err(assembly_error(line, "invalid integer"))
        }
        "jmp" | "jz" if !valid_label(tokens[1]) => {
            Err(assembly_error(line, "invalid label"))
        }
        _ => Ok(()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (index, raw_line) in src.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !valid_label(name) {
                return Err(assembly_error(line_number, "invalid label"));
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err(assembly_error(line_number, "duplicate label"));
            }

            continue;
        }

        validate_instruction(&tokens, line_number)?;
        instruction_count = instruction_count
            .checked_add(1)
            .ok_or_else(|| assembly_error(line_number, "too many instructions"))?;
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (index, raw_line) in src.lines().enumerate() {
        let line_number = index + 1;
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
                    .ok_or_else(|| assembly_error(line_number, "invalid integer"))?,
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
                    .ok_or_else(|| assembly_error(line_number, "undefined label"))?,
            ),
            "jz" => Op::Jz(
                labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| assembly_error(line_number, "undefined label"))?,
            ),
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => return Err(assembly_error(line_number, "unknown mnemonic")),
        };

        ops.push(op);
    }

    Ok(ops)
}
