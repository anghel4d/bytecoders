fn assemble_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn assemble_check_arity(tokens: &[&str], operands: usize) -> Result<(), String> {
    let expected = operands + 1;

    if tokens.len() < expected {
        Err("missing operand".to_string())
    } else if tokens.len() > expected {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut lines = Vec::new();
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("invalid label".to_string());
            }

            let name = &tokens[0][..tokens[0].len() - 1];

            if !assemble_label_name(name) {
                return Err("invalid label".to_string());
            }

            if labels
                .insert(name.to_string(), instruction_count)
                .is_some()
            {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count += 1;
        }

        lines.push(line);
    }

    let mut output = Vec::with_capacity(instruction_count);

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                assemble_check_arity(&tokens, 1)?;
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid integer".to_string())?;
                Op::Push(value)
            }
            "pop" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Pop
            }
            "add" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Add
            }
            "sub" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Sub
            }
            "mul" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Mul
            }
            "div" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Div
            }
            "mod" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Mod
            }
            "neg" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Neg
            }
            "dup" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Dup
            }
            "swap" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Swap
            }
            "jmp" => {
                assemble_check_arity(&tokens, 1)?;
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jmp(target)
            }
            "jz" => {
                assemble_check_arity(&tokens, 1)?;
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jz(target)
            }
            "print" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Print
            }
            "halt" => {
                assemble_check_arity(&tokens, 0)?;
                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        output.push(op);
    }

    Ok(output)
}
