fn is_label_name(name: &str) -> bool {
    let bytes = name.as_bytes();

    if bytes.is_empty()
        || (!bytes[0].is_ascii_alphabetic() && bytes[0] != b'_')
    {
        return false;
    }

    bytes[1..]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
}

fn label_from_line(line: &str) -> Result<Option<&str>, String> {
    if line.ends_with(':') {
        let name = &line[..line.len() - 1];
        if is_label_name(name) {
            Ok(Some(name))
        } else {
            Err("invalid label".to_string())
        }
    } else if line.contains(':') {
        Err("invalid label".to_string())
    } else {
        Ok(None)
    }
}

fn one_operand<'a>(
    tokens: &mut std::str::SplitWhitespace<'a>,
) -> Result<&'a str, String> {
    let operand = tokens
        .next()
        .ok_or_else(|| "missing operand".to_string())?;

    if tokens.next().is_some() {
        return Err("trailing tokens".to_string());
    }

    Ok(operand)
}

fn no_operands(
    tokens: &mut std::str::SplitWhitespace<'_>,
) -> Result<(), String> {
    if tokens.next().is_some() {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}

fn is_decimal_integer(value: &str) -> bool {
    let bytes = value.as_bytes();
    let start = if bytes.first() == Some(&b'-') { 1 } else { 0 };

    start < bytes.len() && bytes[start..].iter().all(|byte| byte.is_ascii_digit())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = label_from_line(line)? {
            if labels.insert(name, instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut code = Vec::with_capacity(instruction_count);

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap().trim();

        if line.is_empty() {
            continue;
        }

        if label_from_line(line)?.is_some() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        let instruction = match mnemonic {
            "push" => {
                let operand = one_operand(&mut tokens)?;
                if !is_decimal_integer(operand) {
                    return Err("invalid operand".to_string());
                }

                let value = operand
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;

                Op::Push(value)
            }
            "pop" => {
                no_operands(&mut tokens)?;
                Op::Pop
            }
            "add" => {
                no_operands(&mut tokens)?;
                Op::Add
            }
            "sub" => {
                no_operands(&mut tokens)?;
                Op::Sub
            }
            "mul" => {
                no_operands(&mut tokens)?;
                Op::Mul
            }
            "div" => {
                no_operands(&mut tokens)?;
                Op::Div
            }
            "mod" => {
                no_operands(&mut tokens)?;
                Op::Mod
            }
            "neg" => {
                no_operands(&mut tokens)?;
                Op::Neg
            }
            "dup" => {
                no_operands(&mut tokens)?;
                Op::Dup
            }
            "swap" => {
                no_operands(&mut tokens)?;
                Op::Swap
            }
            "jmp" => {
                let operand = one_operand(&mut tokens)?;
                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                Op::Jmp(target)
            }
            "jz" => {
                let operand = one_operand(&mut tokens)?;
                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| "undefined label".to_string())?;

                Op::Jz(target)
            }
            "print" => {
                no_operands(&mut tokens)?;
                Op::Print
            }
            "halt" => {
                no_operands(&mut tokens)?;
                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        code.push(instruction);
    }

    Ok(code)
}
