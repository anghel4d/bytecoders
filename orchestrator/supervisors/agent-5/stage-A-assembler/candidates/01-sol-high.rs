fn is_label_name(name: &str) -> bool {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') => {}
        _ => return false,
    }

    bytes.all(|byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}

fn parse_integer(value: &str, line: usize) -> Result<i64, String> {
    let digits = value.strip_prefix('-').unwrap_or(value);

    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(format!("line {line}: invalid integer"));
    }

    value
        .parse()
        .map_err(|_| format!("line {line}: invalid integer"))
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut lines = Vec::new();
    let mut instruction_index = 0;

    for (offset, source_line) in src.lines().enumerate() {
        let line_number = offset + 1;
        let line = source_line
            .split_once(';')
            .map_or(source_line, |(code, _)| code)
            .trim();

        if line.is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let first = tokens.next().unwrap();

        if let Some(name) = first.strip_suffix(':') {
            if tokens.next().is_some() {
                return Err(format!("line {line_number}: trailing tokens"));
            }
            if !is_label_name(name) {
                return Err(format!("line {line_number}: invalid label"));
            }
            if labels.insert(name, instruction_index).is_some() {
                return Err(format!("line {line_number}: duplicate label"));
            }
        } else {
            lines.push((line_number, line));
            instruction_index += 1;
        }
    }

    let mut ops = Vec::with_capacity(lines.len());

    for (line_number, line) in lines {
        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        let op = match mnemonic {
            "push" => {
                let value = tokens
                    .next()
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?;
                if tokens.next().is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                Op::Push(parse_integer(value, line_number)?)
            }
            "jmp" | "jz" => {
                let label = tokens
                    .next()
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?;
                if tokens.next().is_some() {
                    return Err(format!("line {line_number}: trailing tokens"));
                }
                if !is_label_name(label) {
                    return Err(format!("line {line_number}: invalid operand"));
                }
                let target = *labels
                    .get(label)
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
            | "print" | "halt" => {
                if tokens.next().is_some() {
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
