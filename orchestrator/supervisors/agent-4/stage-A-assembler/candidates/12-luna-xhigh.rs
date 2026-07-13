fn assemble_valid_label(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    let first = bytes[0];
    if !(first == b'_' || first.is_ascii_alphabetic()) {
        return false;
    }

    bytes[1..]
        .iter()
        .all(|&byte| byte == b'_' || byte.is_ascii_alphanumeric())
}

fn assemble_parse_int(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len()
        || !bytes[start..]
            .iter()
            .all(|&byte| byte >= b'0' && byte <= b'9')
    {
        return None;
    }

    value.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instructions = Vec::new();
    let mut next_index = 0usize;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap().trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !assemble_valid_label(name) {
                return Err(format!("line {}: invalid label", line_number));
            }
            if labels.insert(name, next_index).is_some() {
                return Err(format!("line {}: duplicate label", line_number));
            }
        } else {
            if tokens[0].ends_with(':') {
                return Err(format!("line {}: trailing tokens", line_number));
            }

            instructions.push((line_number, line));
            next_index += 1;
        }
    }

    let mut output = Vec::with_capacity(next_index);

    for (line_number, line) in instructions {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        match tokens[0] {
            "push" => {
                if tokens.len() != 2 {
                    return Err(format!("line {}: push expects an integer", line_number));
                }

                let value = assemble_parse_int(tokens[1])
                    .ok_or_else(|| format!("line {}: invalid integer", line_number))?;
                output.push(Op::Push(value));
            }
            "jmp" | "jz" => {
                if tokens.len() != 2 {
                    return Err(format!("line {}: jump expects a label", line_number));
                }

                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| format!("line {}: undefined label", line_number))?;

                if tokens[0] == "jmp" {
                    output.push(Op::Jmp(target));
                } else {
                    output.push(Op::Jz(target));
                }
            }
            "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
            | "print" | "halt" => {
                if tokens.len() != 1 {
                    return Err(format!("line {}: trailing tokens", line_number));
                }

                output.push(match tokens[0] {
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
                });
            }
            _ => {
                return Err(format!("line {}: unknown mnemonic", line_number));
            }
        }
    }

    Ok(output)
}
