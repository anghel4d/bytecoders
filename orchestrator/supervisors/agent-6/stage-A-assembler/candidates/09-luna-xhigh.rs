enum AssembleItem {
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

fn assemble_is_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn assemble_parse_int(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    let start = usize::from(bytes.first() == Some(&b'-'));

    if start == bytes.len() || !bytes[start..].iter().all(u8::is_ascii_digit) {
        return None;
    }

    value.parse().ok()
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut items = Vec::new();

    for line in src.lines() {
        let line = line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }

            let name = &tokens[0][..tokens[0].len() - 1];

            if !assemble_is_name(name) {
                return Err("invalid label".to_string());
            }

            if labels.insert(name.to_string(), items.len()).is_some() {
                return Err("duplicate label".to_string());
            }

            continue;
        }

        match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }

                let value =
                    assemble_parse_int(tokens[1]).ok_or_else(|| "invalid operand".to_string())?;
                items.push(AssembleItem::Push(value));
            }
            "jmp" | "jz" => {
                if tokens.len() < 2 {
                    return Err("missing operand".to_string());
                }
                if tokens.len() > 2 {
                    return Err("trailing tokens".to_string());
                }
                if !assemble_is_name(tokens[1]) {
                    return Err("invalid operand".to_string());
                }

                let label = tokens[1].to_string();
                if tokens[0] == "jmp" {
                    items.push(AssembleItem::Jmp(label));
                } else {
                    items.push(AssembleItem::Jz(label));
                }
            }
            "pop" => items.push(AssembleItem::Pop),
            "add" => items.push(AssembleItem::Add),
            "sub" => items.push(AssembleItem::Sub),
            "mul" => items.push(AssembleItem::Mul),
            "div" => items.push(AssembleItem::Div),
            "mod" => items.push(AssembleItem::Mod),
            "neg" => items.push(AssembleItem::Neg),
            "dup" => items.push(AssembleItem::Dup),
            "swap" => items.push(AssembleItem::Swap),
            "print" => items.push(AssembleItem::Print),
            "halt" => items.push(AssembleItem::Halt),
            _ => return Err("unknown mnemonic".to_string()),
        }

        if tokens[0] != "push"
            && tokens[0] != "jmp"
            && tokens[0] != "jz"
            && tokens.len() > 1
        {
            return Err("trailing tokens".to_string());
        }
    }

    items
        .into_iter()
        .map(|item| match item {
            AssembleItem::Push(value) => Ok(Op::Push(value)),
            AssembleItem::Pop => Ok(Op::Pop),
            AssembleItem::Add => Ok(Op::Add),
            AssembleItem::Sub => Ok(Op::Sub),
            AssembleItem::Mul => Ok(Op::Mul),
            AssembleItem::Div => Ok(Op::Div),
            AssembleItem::Mod => Ok(Op::Mod),
            AssembleItem::Neg => Ok(Op::Neg),
            AssembleItem::Dup => Ok(Op::Dup),
            AssembleItem::Swap => Ok(Op::Swap),
            AssembleItem::Jmp(label) => labels
                .get(&label)
                .copied()
                .map(Op::Jmp)
                .ok_or_else(|| "undefined label".to_string()),
            AssembleItem::Jz(label) => labels
                .get(&label)
                .copied()
                .map(Op::Jz)
                .ok_or_else(|| "undefined label".to_string()),
            AssembleItem::Print => Ok(Op::Print),
            AssembleItem::Halt => Ok(Op::Halt),
        })
        .collect()
}
