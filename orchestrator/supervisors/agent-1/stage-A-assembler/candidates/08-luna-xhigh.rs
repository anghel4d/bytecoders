fn assembly_label_is_valid(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn validate_assembly_instruction(tokens: &[&str]) -> Result<(), String> {
    let expected = match tokens[0] {
        "push" | "jmp" | "jz" => 2,
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
        | "print" | "halt" => 1,
        _ => return Err("unknown mnemonic".to_string()),
    };

    if tokens.len() < expected {
        return Err("missing operand".to_string());
    }
    if tokens.len() > expected {
        return Err("trailing tokens".to_string());
    }

    match tokens[0] {
        "push" => {
            tokens[1]
                .parse::<i64>()
                .map_err(|_| "invalid operand".to_string())?;
        }
        "jmp" | "jz" => {
            if !assembly_label_is_valid(tokens[1]) {
                return Err("invalid operand".to_string());
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<&str, usize>::new();
    let mut lines: Vec<Vec<&str>> = Vec::new();

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }

            let name = &tokens[0][..tokens[0].len() - 1];
            if !assembly_label_is_valid(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, lines.len()).is_some() {
                return Err("duplicate label".to_string());
            }
            continue;
        }

        validate_assembly_instruction(&tokens)?;
        lines.push(tokens);
    }

    let mut output = Vec::with_capacity(lines.len());

    for tokens in lines {
        let op = match tokens[0] {
            "push" => Op::Push(
                tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?,
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
                *labels
                    .get(tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?,
            ),
            "jz" => Op::Jz(
                *labels
                    .get(tokens[1])
                    .ok_or_else(|| "undefined label".to_string())?,
            ),
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => return Err("unknown mnemonic".to_string()),
        };
        output.push(op);
    }

    Ok(output)
}
