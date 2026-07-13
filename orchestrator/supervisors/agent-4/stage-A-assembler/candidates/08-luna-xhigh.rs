pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    enum Pending {
        Push(i64),
        Pop,
        Add,
        Sub,
        Mul,
        Div,
        Mod,
        Neg,
        Dup,
        Swap,
        Jmp(String),
        Jz(String),
        Print,
        Halt,
    }

    let valid_label = |name: &str| {
        let mut chars = name.chars();
        match chars.next() {
            Some(c) if c == '_' || c.is_ascii_alphabetic() => {}
            _ => return false,
        }
        chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
    };

    let parse_int = |value: &str| -> Result<i64, ()> {
        let bytes = value.as_bytes();
        let start = usize::from(bytes.first() == Some(&b'-'));
        if start == bytes.len()
            || !bytes[start..]
                .iter()
                .all(|byte| b'0' <= *byte && *byte <= b'9')
        {
            return Err(());
        }
        value.parse::<i64>().map_err(|_| ())
    };

    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut pending = Vec::new();

    for raw_line in src.lines() {
        let line = match raw_line.find(';') {
            Some(index) => &raw_line[..index],
            None => raw_line,
        };

        let mut tokens = line.split_whitespace();
        let Some(mnemonic) = tokens.next() else {
            continue;
        };
        let operand = tokens.next();

        if tokens.next().is_some() {
            return Err("trailing tokens".to_string());
        }

        if mnemonic.ends_with(':') {
            let name = &mnemonic[..mnemonic.len() - 1];
            if operand.is_some() {
                return Err("trailing tokens".to_string());
            }
            if !valid_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name.to_string(), pending.len()).is_some() {
                return Err("duplicate label".to_string());
            }
            continue;
        }

        match mnemonic {
            "push" => {
                let value = operand.ok_or_else(|| "missing operand".to_string())?;
                let value = parse_int(value).map_err(|_| "invalid integer".to_string())?;
                pending.push(Pending::Push(value));
            }
            "pop" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Pop);
            }
            "add" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Add);
            }
            "sub" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Sub);
            }
            "mul" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Mul);
            }
            "div" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Div);
            }
            "mod" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Mod);
            }
            "neg" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Neg);
            }
            "dup" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Dup);
            }
            "swap" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Swap);
            }
            "jmp" => {
                let label = operand.ok_or_else(|| "missing operand".to_string())?;
                if !valid_label(label) {
                    return Err("invalid label".to_string());
                }
                pending.push(Pending::Jmp(label.to_string()));
            }
            "jz" => {
                let label = operand.ok_or_else(|| "missing operand".to_string())?;
                if !valid_label(label) {
                    return Err("invalid label".to_string());
                }
                pending.push(Pending::Jz(label.to_string()));
            }
            "print" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Print);
            }
            "halt" => {
                if operand.is_some() {
                    return Err("trailing tokens".to_string());
                }
                pending.push(Pending::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    let mut ops = Vec::with_capacity(pending.len());

    for instruction in pending {
        let op = match instruction {
            Pending::Push(value) => Op::Push(value),
            Pending::Pop => Op::Pop,
            Pending::Add => Op::Add,
            Pending::Sub => Op::Sub,
            Pending::Mul => Op::Mul,
            Pending::Div => Op::Div,
            Pending::Mod => Op::Mod,
            Pending::Neg => Op::Neg,
            Pending::Dup => Op::Dup,
            Pending::Swap => Op::Swap,
            Pending::Jmp(label) => {
                let target = *labels
                    .get(&label)
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jmp(target)
            }
            Pending::Jz(label) => {
                let target = *labels
                    .get(&label)
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jz(target)
            }
            Pending::Print => Op::Print,
            Pending::Halt => Op::Halt,
        };
        ops.push(op);
    }

    Ok(ops)
}
