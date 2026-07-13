fn assemble_label_is_valid(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
        return false;
    }
    bytes[1..]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
}

fn assemble_check_arity(tokens: &[&str], expected: usize) -> Result<(), String> {
    match tokens.len().cmp(&expected) {
        std::cmp::Ordering::Less => Err("missing operand".to_string()),
        std::cmp::Ordering::Greater => Err("trailing tokens".to_string()),
        std::cmp::Ordering::Equal => Ok(()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_lines = Vec::new();
    let mut instruction_index = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }

            let name = &tokens[0][..tokens[0].len() - 1];
            if !assemble_label_is_valid(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name, instruction_index).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_lines.push(tokens);
            instruction_index += 1;
        }
    }

    let mut output = Vec::with_capacity(instruction_index);

    for tokens in instruction_lines {
        let mnemonic = tokens[0];
        let op = match mnemonic {
            "push" => {
                assemble_check_arity(&tokens, 2)?;
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;
                Op::Push(value)
            }
            "pop" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Pop
            }
            "add" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Add
            }
            "sub" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Sub
            }
            "mul" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Mul
            }
            "div" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Div
            }
            "mod" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Mod
            }
            "neg" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Neg
            }
            "dup" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Dup
            }
            "swap" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Swap
            }
            "jmp" => {
                assemble_check_arity(&tokens, 2)?;
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jmp(target)
            }
            "jz" => {
                assemble_check_arity(&tokens, 2)?;
                let target = labels
                    .get(tokens[1])
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jz(target)
            }
            "print" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Print
            }
            "halt" => {
                assemble_check_arity(&tokens, 1)?;
                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };
        output.push(op);
    }

    Ok(output)
}
