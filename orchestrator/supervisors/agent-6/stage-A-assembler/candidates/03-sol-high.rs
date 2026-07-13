fn valid_label(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'_'))
        && bytes.all(|byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}

fn one_operand<'a>(operands: &'a [&'a str]) -> Result<&'a str, String> {
    match operands {
        [operand] => Ok(operand),
        [] => Err("missing operand".to_string()),
        _ => Err("trailing tokens".to_string()),
    }
}

fn no_operands(operands: &[&str]) -> Result<(), String> {
    if operands.is_empty() {
        Ok(())
    } else {
        Err("trailing tokens".to_string())
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_index = 0;

    for (line_number, raw_line) in src.lines().enumerate() {
        let line = raw_line
            .split_once(';')
            .map_or(raw_line, |(code, _)| code)
            .trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !valid_label(name) {
                return Err(format!("line {}: invalid label", line_number + 1));
            }
            if labels.insert(name, instruction_index).is_some() {
                return Err(format!("line {}: duplicate label", line_number + 1));
            }
        } else {
            instruction_index += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_index);

    for (line_number, raw_line) in src.lines().enumerate() {
        let line = raw_line
            .split_once(';')
            .map_or(raw_line, |(code, _)| code)
            .trim();

        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();
        let operands: Vec<&str> = tokens.collect();
        let error = |message: String| format!("line {}: {}", line_number + 1, message);

        let op = match mnemonic {
            "push" => {
                let operand = one_operand(&operands).map_err(&error)?;
                let value = operand
                    .parse::<i64>()
                    .map_err(|_| error("invalid operand".to_string()))?;
                Op::Push(value)
            }
            "pop" => {
                no_operands(&operands).map_err(&error)?;
                Op::Pop
            }
            "add" => {
                no_operands(&operands).map_err(&error)?;
                Op::Add
            }
            "sub" => {
                no_operands(&operands).map_err(&error)?;
                Op::Sub
            }
            "mul" => {
                no_operands(&operands).map_err(&error)?;
                Op::Mul
            }
            "div" => {
                no_operands(&operands).map_err(&error)?;
                Op::Div
            }
            "mod" => {
                no_operands(&operands).map_err(&error)?;
                Op::Mod
            }
            "neg" => {
                no_operands(&operands).map_err(&error)?;
                Op::Neg
            }
            "dup" => {
                no_operands(&operands).map_err(&error)?;
                Op::Dup
            }
            "swap" => {
                no_operands(&operands).map_err(&error)?;
                Op::Swap
            }
            "jmp" | "jz" => {
                let label = one_operand(&operands).map_err(&error)?;
                if !valid_label(label) {
                    return Err(error("invalid operand".to_string()));
                }
                let target = labels
                    .get(label)
                    .copied()
                    .ok_or_else(|| error(format!("undefined label {}", label)))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => {
                no_operands(&operands).map_err(&error)?;
                Op::Print
            }
            "halt" => {
                no_operands(&operands).map_err(&error)?;
                Op::Halt
            }
            _ => return Err(error(format!("unknown mnemonic {}", mnemonic))),
        };

        ops.push(op);
    }

    Ok(ops)
}
