fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_'))
}

fn label_definition(line: &str) -> Result<Option<&str>, String> {
    let mut tokens = line.split_whitespace();
    let Some(token) = tokens.next() else {
        return Ok(None);
    };

    if tokens.next().is_none() {
        if let Some(name) = token.strip_suffix(':') {
            if is_label_name(name) {
                return Ok(Some(name));
            }
            return Err("invalid label".to_string());
        }
    }

    Ok(None)
}

fn operand<'a>(
    tokens: &mut std::str::SplitWhitespace<'a>,
    mnemonic: &str,
) -> Result<&'a str, String> {
    tokens
        .next()
        .ok_or_else(|| format!("missing operand for {}", mnemonic))
}

fn reject_trailing(
    tokens: &mut std::str::SplitWhitespace<'_>,
) -> Result<(), String> {
    if tokens.next().is_some() {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_index = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }

        if let Some(name) = label_definition(line)? {
            if labels.insert(name, instruction_index).is_some() {
                return Err(format!("duplicate label: {}", name));
            }
        } else {
            instruction_index += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_index);

    for raw_line in src.lines() {
        let line = raw_line.split_once(';').map_or(raw_line, |(code, _)| code).trim();
        if line.is_empty() || label_definition(line)?.is_some() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        let op = match mnemonic {
            "push" => {
                let value = operand(&mut tokens, mnemonic)?
                    .parse::<i64>()
                    .map_err(|_| "invalid push operand".to_string())?;
                reject_trailing(&mut tokens)?;
                Op::Push(value)
            }
            "jmp" | "jz" => {
                let name = operand(&mut tokens, mnemonic)?;
                if !is_label_name(name) {
                    return Err(format!("invalid {} operand", mnemonic));
                }
                reject_trailing(&mut tokens)?;
                let target = labels
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("undefined label: {}", name))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "pop" => {
                reject_trailing(&mut tokens)?;
                Op::Pop
            }
            "add" => {
                reject_trailing(&mut tokens)?;
                Op::Add
            }
            "sub" => {
                reject_trailing(&mut tokens)?;
                Op::Sub
            }
            "mul" => {
                reject_trailing(&mut tokens)?;
                Op::Mul
            }
            "div" => {
                reject_trailing(&mut tokens)?;
                Op::Div
            }
            "mod" => {
                reject_trailing(&mut tokens)?;
                Op::Mod
            }
            "neg" => {
                reject_trailing(&mut tokens)?;
                Op::Neg
            }
            "dup" => {
                reject_trailing(&mut tokens)?;
                Op::Dup
            }
            "swap" => {
                reject_trailing(&mut tokens)?;
                Op::Swap
            }
            "print" => {
                reject_trailing(&mut tokens)?;
                Op::Print
            }
            "halt" => {
                reject_trailing(&mut tokens)?;
                Op::Halt
            }
            _ => return Err(format!("unknown mnemonic: {}", mnemonic)),
        };

        ops.push(op);
    }

    Ok(ops)
}
