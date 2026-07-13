fn is_assemble_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn assemble_operand<'a>(tokens: &[&'a str]) -> Result<&'a str, String> {
    match tokens.len() {
        1 => Err("missing operand".to_string()),
        2 => Ok(tokens[1]),
        _ => Err("trailing tokens".to_string()),
    }
}

fn assemble_no_operands(tokens: &[&str]) -> Result<(), String> {
    if tokens.len() == 1 {
        Ok(())
    } else {
        Err("trailing tokens".to_string())
    }
}

fn parse_assemble_int(text: &str) -> Result<i64, String> {
    let bytes = text.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    if start == bytes.len()
        || !bytes[start..]
            .iter()
            .all(|byte| *byte >= b'0' && *byte <= b'9')
    {
        return Err("invalid operand".to_string());
    }

    text.parse::<i64>()
        .map_err(|_| "invalid operand".to_string())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::<String, usize>::new();
    let mut instruction_count = 0usize;

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
            if !is_assemble_label(name) {
                return Err("invalid label".to_string());
            }

            if labels.insert(name.to_string(), instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut output = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens[0].ends_with(':') {
            continue;
        }

        match tokens[0] {
            "push" => {
                let operand = assemble_operand(&tokens)?;
                output.push(Op::Push(parse_assemble_int(operand)?));
            }
            "pop" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Pop);
            }
            "add" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Add);
            }
            "sub" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Sub);
            }
            "mul" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Mul);
            }
            "div" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Div);
            }
            "mod" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Mod);
            }
            "neg" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Neg);
            }
            "dup" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Dup);
            }
            "swap" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Swap);
            }
            "jmp" => {
                let operand = assemble_operand(&tokens)?;
                if !is_assemble_label(operand) {
                    return Err("invalid operand".to_string());
                }

                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                output.push(Op::Jmp(target));
            }
            "jz" => {
                let operand = assemble_operand(&tokens)?;
                if !is_assemble_label(operand) {
                    return Err("invalid operand".to_string());
                }

                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                output.push(Op::Jz(target));
            }
            "print" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Print);
            }
            "halt" => {
                assemble_no_operands(&tokens)?;
                output.push(Op::Halt);
            }
            _ => return Err("unknown mnemonic".to_string()),
        }
    }

    Ok(output)
}
