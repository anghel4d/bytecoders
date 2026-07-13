fn is_label_name(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some('A'..='Z' | 'a'..='z' | '_') => {}
        _ => return false,
    }
    chars.all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_'))
}

fn parse_integer(s: &str) -> Option<i64> {
    let digits = s.strip_prefix('-').unwrap_or(s);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

fn check_arity(tokens: &[&str], operands: usize) -> Result<(), &'static str> {
    if tokens.len() < operands + 1 {
        Err("missing operand")
    } else if tokens.len() > operands + 1 {
        Err("trailing tokens")
    } else {
        Ok(())
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (line_index, line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let code = line.split_once(';').map_or(line, |(code, _)| code);
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if let Some(name) = tokens[0].strip_suffix(':') {
            if !is_label_name(name) {
                return Err(format!("line {line_number}: invalid label"));
            }
            if tokens.len() != 1 {
                return Err(format!("line {line_number}: trailing tokens"));
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err(format!("line {line_number}: duplicate label"));
            }
            continue;
        }

        let result = match tokens[0] {
            "push" => {
                check_arity(&tokens, 1)?;
                if parse_integer(tokens[1]).is_none() {
                    Err("invalid operand")
                } else {
                    Ok(())
                }
            }
            "jmp" | "jz" => {
                check_arity(&tokens, 1)?;
                if !is_label_name(tokens[1]) {
                    Err("invalid operand")
                } else {
                    Ok(())
                }
            }
            "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup"
            | "swap" | "print" | "halt" => check_arity(&tokens, 0),
            _ => Err("unknown mnemonic"),
        };

        result.map_err(|message| format!("line {line_number}: {message}"))?;
        instruction_count += 1;
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_index, line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let code = line.split_once(';').map_or(line, |(code, _)| code);
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.is_empty() || tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => Op::Push(
                parse_integer(tokens[1])
                    .ok_or_else(|| format!("line {line_number}: invalid operand"))?,
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
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?,
            ),
            "jz" => Op::Jz(
                *labels
                    .get(tokens[1])
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?,
            ),
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        ops.push(op);
    }

    Ok(ops)
}
