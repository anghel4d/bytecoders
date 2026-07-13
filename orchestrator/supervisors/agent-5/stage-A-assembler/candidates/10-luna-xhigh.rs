fn valid_label(name: &str) -> bool {
    let bytes = name.as_bytes();

    if bytes.is_empty()
        || !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_')
    {
        return false;
    }

    bytes[1..]
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
}

fn require_no_operand(tokens: &[&str]) -> Result<(), String> {
    if tokens.len() == 1 {
        Ok(())
    } else {
        Err("trailing tokens".to_string())
    }
}

fn require_operand<'a>(tokens: &[&'a str]) -> Result<&'a str, String> {
    match tokens.len() {
        1 => Err("missing operand".to_string()),
        2 => Ok(tokens[1]),
        _ => Err("trailing tokens".to_string()),
    }
}

fn validate_instruction(tokens: &[&str]) -> Result<(), String> {
    match tokens[0] {
        "push" => {
            let operand = require_operand(tokens)?;
            operand
                .parse::<i64>()
                .map_err(|_| "invalid operand".to_string())?;
            Ok(())
        }
        "jmp" | "jz" => {
            let operand = require_operand(tokens)?;
            if valid_label(operand) {
                Ok(())
            } else {
                Err("invalid operand".to_string())
            }
        }
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
        | "print" | "halt" => require_no_operand(tokens),
        _ => Err("unknown mnemonic".to_string()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut lines = Vec::new();
    let mut instruction_count = 0usize;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        let tokens = line.split_whitespace().collect::<Vec<_>>();

        if tokens[0].ends_with(':') {
            if tokens.len() != 1 {
                return Err("trailing tokens".to_string());
            }

            let name = &tokens[0][..tokens[0].len() - 1];
            if !valid_label(name) {
                return Err("invalid label".to_string());
            }
            if labels.insert(name.to_string(), instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }

            lines.push(tokens);
        } else {
            validate_instruction(&tokens)?;
            instruction_count += 1;
            lines.push(tokens);
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for tokens in &lines {
        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                let value = require_operand(tokens)?
                    .parse::<i64>()
                    .map_err(|_| "invalid operand".to_string())?;
                Op::Push(value)
            }
            "pop" => {
                require_no_operand(tokens)?;
                Op::Pop
            }
            "add" => {
                require_no_operand(tokens)?;
                Op::Add
            }
            "sub" => {
                require_no_operand(tokens)?;
                Op::Sub
            }
            "mul" => {
                require_no_operand(tokens)?;
                Op::Mul
            }
            "div" => {
                require_no_operand(tokens)?;
                Op::Div
            }
            "mod" => {
                require_no_operand(tokens)?;
                Op::Mod
            }
            "neg" => {
                require_no_operand(tokens)?;
                Op::Neg
            }
            "dup" => {
                require_no_operand(tokens)?;
                Op::Dup
            }
            "swap" => {
                require_no_operand(tokens)?;
                Op::Swap
            }
            "jmp" => {
                let label = require_operand(tokens)?;
                let target = *labels
                    .get(label)
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jmp(target)
            }
            "jz" => {
                let label = require_operand(tokens)?;
                let target = *labels
                    .get(label)
                    .ok_or_else(|| "undefined label".to_string())?;
                Op::Jz(target)
            }
            "print" => {
                require_no_operand(tokens)?;
                Op::Print
            }
            "halt" => {
                require_no_operand(tokens)?;
                Op::Halt
            }
            _ => return Err("unknown mnemonic".to_string()),
        };

        ops.push(op);
    }

    Ok(ops)
}
