fn valid_label_name(name: &str) -> bool {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(b'A'..=b'Z') | Some(b'a'..=b'z') | Some(b'_') => {}
        _ => return false,
    }

    bytes.all(|byte| {
        matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_')
    })
}

fn validate_instruction(tokens: &[&str]) -> Result<(), String> {
    match tokens[0] {
        "push" => {
            if tokens.len() != 2 {
                return Err("invalid operands".to_string());
            }
            tokens[1]
                .parse::<i64>()
                .map(|_| ())
                .map_err(|_| "invalid integer".to_string())
        }
        "jmp" | "jz" => {
            if tokens.len() != 2 {
                return Err("invalid operands".to_string());
            }
            if valid_label_name(tokens[1]) {
                Ok(())
            } else {
                Err("invalid label".to_string())
            }
        }
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
        | "print" | "halt" => {
            if tokens.len() == 1 {
                Ok(())
            } else {
                Err("trailing tokens".to_string())
            }
        }
        _ => Err("unknown mnemonic".to_string()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];
            if !valid_label_name(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name.to_string(), instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            validate_instruction(&tokens)?;
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        validate_instruction(&tokens)?;

        let op = match tokens[0] {
            "push" => Op::Push(
                tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid integer".to_string())?,
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

        ops.push(op);
    }

    Ok(ops)
}
