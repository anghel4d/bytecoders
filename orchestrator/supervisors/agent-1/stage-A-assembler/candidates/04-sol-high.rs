fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn decimal_i64(token: &str) -> Option<i64> {
    let digits = token.strip_prefix('-').unwrap_or(token);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    token.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instructions = Vec::new();

    for line in src.lines() {
        let line = line.split_once(';').map_or(line, |(code, _)| code).trim();
        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !valid_label(name) {
                return Err("invalid label".into());
            }
            if labels.insert(name, instructions.len()).is_some() {
                return Err("duplicate label".into());
            }
        } else {
            instructions.push(line);
        }
    }

    let mut ops = Vec::with_capacity(instructions.len());

    for line in instructions {
        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();
        let operand = tokens.next();

        if tokens.next().is_some() {
            return Err("trailing tokens".into());
        }

        let op = match mnemonic {
            "push" => {
                let value = operand
                    .and_then(decimal_i64)
                    .ok_or_else(|| "invalid push operand".to_string())?;
                Op::Push(value)
            }
            "pop" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Pop
            }
            "add" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Add
            }
            "sub" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Sub
            }
            "mul" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Mul
            }
            "div" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Div
            }
            "mod" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Mod
            }
            "neg" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Neg
            }
            "dup" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Dup
            }
            "swap" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Swap
            }
            "jmp" | "jz" => {
                let name = operand.ok_or_else(|| "missing label".to_string())?;
                if !valid_label(name) {
                    return Err("invalid label".into());
                }
                let target = *labels
                    .get(name)
                    .ok_or_else(|| "undefined label".to_string())?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Print
            }
            "halt" => {
                if operand.is_some() {
                    return Err("trailing tokens".into());
                }
                Op::Halt
            }
            _ => return Err("unknown mnemonic".into()),
        };

        ops.push(op);
    }

    Ok(ops)
}
