fn take_operand<'a>(
    tokens: &mut std::str::SplitWhitespace<'a>,
    mnemonic: &str,
) -> Result<&'a str, String> {
    let operand = tokens
        .next()
        .ok_or_else(|| format!("{} missing operand", mnemonic))?;
    if tokens.next().is_some() {
        return Err(format!("{} has trailing tokens", mnemonic));
    }
    Ok(operand)
}

fn reject_operands(
    tokens: &mut std::str::SplitWhitespace<'_>,
    mnemonic: &str,
) -> Result<(), String> {
    if tokens.next().is_some() {
        return Err(format!("{} has trailing tokens", mnemonic));
    }
    Ok(())
}

fn valid_label_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_integer(token: &str) -> Result<i64, String> {
    let bytes = token.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len() || !bytes[start..].iter().all(u8::is_ascii_digit) {
        return Err("invalid integer".to_string());
    }

    token
        .parse::<i64>()
        .map_err(|_| "invalid integer".to_string())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instruction_lines = Vec::new();
    let mut instruction_index = 0;

    for raw_line in src.lines() {
        let line = raw_line
            .split_once(';')
            .map_or(raw_line, |(code, _)| code)
            .trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !valid_label_name(name) {
                return Err("invalid label".to_string());
            }
            if labels
                .insert(name.to_string(), instruction_index)
                .is_some()
            {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_lines.push(line);
            instruction_index += 1;
        }
    }

    let mut program = Vec::with_capacity(instruction_lines.len());

    for line in instruction_lines {
        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        let op = match mnemonic {
            "push" => Op::Push(parse_integer(take_operand(&mut tokens, mnemonic)?)?),
            "pop" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Pop
            }
            "add" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Add
            }
            "sub" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Sub
            }
            "mul" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Mul
            }
            "div" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Div
            }
            "mod" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Mod
            }
            "neg" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Neg
            }
            "dup" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Dup
            }
            "swap" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Swap
            }
            "jmp" => {
                let label = take_operand(&mut tokens, mnemonic)?;
                let target = labels
                    .get(label)
                    .copied()
                    .ok_or_else(|| format!("undefined label: {}", label))?;
                Op::Jmp(target)
            }
            "jz" => {
                let label = take_operand(&mut tokens, mnemonic)?;
                let target = labels
                    .get(label)
                    .copied()
                    .ok_or_else(|| format!("undefined label: {}", label))?;
                Op::Jz(target)
            }
            "print" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Print
            }
            "halt" => {
                reject_operands(&mut tokens, mnemonic)?;
                Op::Halt
            }
            _ => return Err(format!("unknown mnemonic: {}", mnemonic)),
        };

        program.push(op);
    }

    Ok(program)
}
