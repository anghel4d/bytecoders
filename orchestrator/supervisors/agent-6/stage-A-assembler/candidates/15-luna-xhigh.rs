fn is_assemble_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_assemble_int(value: &str) -> Result<i64, String> {
    let digits = value.strip_prefix('-').unwrap_or(value);

    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err("invalid operand".to_string());
    }

    value
        .parse::<i64>()
        .map_err(|_| "invalid operand".to_string())
}

fn validate_assemble_instruction(tokens: &[&str]) -> Result<(), String> {
    let mnemonic = tokens.first().copied().ok_or_else(|| "empty line".to_string())?;

    match mnemonic {
        "push" => {
            if tokens.len() < 2 {
                return Err("missing operand".to_string());
            }
            if tokens.len() > 2 {
                return Err("trailing tokens".to_string());
            }
            parse_assemble_int(tokens[1])?;
        }
        "jmp" | "jz" => {
            if tokens.len() < 2 {
                return Err("missing operand".to_string());
            }
            if tokens.len() > 2 {
                return Err("trailing tokens".to_string());
            }
            if !is_assemble_label(tokens[1]) {
                return Err("invalid operand".to_string());
            }
        }
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
        | "print" | "halt" => {
            if tokens.len() > 1 {
                return Err("trailing tokens".to_string());
            }
        }
        _ => return Err("unknown mnemonic".to_string()),
    }

    Ok(())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels: Vec<(&str, usize)> = Vec::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        let code = line.split(';').next().unwrap_or("").trim();
        if code.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !is_assemble_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.iter().any(|(label, _)| *label == name) {
                return Err("duplicate label".to_string());
            }

            labels.push((name, instruction_count));
        } else {
            validate_assemble_instruction(&tokens)?;
            instruction_count += 1;
        }
    }

    let mut output = Vec::with_capacity(instruction_count);

    for line in src.lines() {
        let code = line.split(';').next().unwrap_or("").trim();
        if code.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        match tokens[0] {
            "push" => output.push(Op::Push(parse_assemble_int(tokens[1])?)),
            "pop" => output.push(Op::Pop),
            "add" => output.push(Op::Add),
            "sub" => output.push(Op::Sub),
            "mul" => output.push(Op::Mul),
            "div" => output.push(Op::Div),
            "mod" => output.push(Op::Mod),
            "neg" => output.push(Op::Neg),
            "dup" => output.push(Op::Dup),
            "swap" => output.push(Op::Swap),
            "jmp" | "jz" => {
                let target = labels
                    .iter()
                    .find(|(label, _)| *label == tokens[1])
                    .map(|(_, index)| *index)
                    .ok_or_else(|| "undefined label".to_string())?;

                if tokens[0] == "jmp" {
                    output.push(Op::Jmp(target));
                } else {
                    output.push(Op::Jz(target));
                }
            }
            "print" => output.push(Op::Print),
            "halt" => output.push(Op::Halt),
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(output)
}
