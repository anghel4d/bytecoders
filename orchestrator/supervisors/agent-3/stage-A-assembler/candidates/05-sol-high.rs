fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_'))
}

fn take_operand<'a>(
    tokens: &mut std::str::SplitWhitespace<'a>,
    line: usize,
) -> Result<&'a str, String> {
    let operand = tokens
        .next()
        .ok_or_else(|| format!("line {line}: missing operand"))?;
    if tokens.next().is_some() {
        return Err(format!("line {line}: trailing tokens"));
    }
    Ok(operand)
}

fn require_end(
    tokens: &mut std::str::SplitWhitespace<'_>,
    line: usize,
) -> Result<(), String> {
    if tokens.next().is_some() {
        return Err(format!("line {line}: trailing tokens"));
    }
    Ok(())
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for (index, raw_line) in src.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !is_label_name(name) {
                return Err(format!("line {line_number}: invalid label"));
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err(format!("line {line_number}: duplicate label"));
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (index, raw_line) in src.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        let op = match mnemonic {
            "push" => {
                let operand = take_operand(&mut tokens, line_number)?;
                let value = operand
                    .parse::<i64>()
                    .map_err(|_| format!("line {line_number}: invalid integer"))?;
                Op::Push(value)
            }
            "pop" => {
                require_end(&mut tokens, line_number)?;
                Op::Pop
            }
            "add" => {
                require_end(&mut tokens, line_number)?;
                Op::Add
            }
            "sub" => {
                require_end(&mut tokens, line_number)?;
                Op::Sub
            }
            "mul" => {
                require_end(&mut tokens, line_number)?;
                Op::Mul
            }
            "div" => {
                require_end(&mut tokens, line_number)?;
                Op::Div
            }
            "mod" => {
                require_end(&mut tokens, line_number)?;
                Op::Mod
            }
            "neg" => {
                require_end(&mut tokens, line_number)?;
                Op::Neg
            }
            "dup" => {
                require_end(&mut tokens, line_number)?;
                Op::Dup
            }
            "swap" => {
                require_end(&mut tokens, line_number)?;
                Op::Swap
            }
            "jmp" | "jz" => {
                let label = take_operand(&mut tokens, line_number)?;
                if !is_label_name(label) {
                    return Err(format!("line {line_number}: invalid label"));
                }
                let target = labels
                    .get(label)
                    .copied()
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => {
                require_end(&mut tokens, line_number)?;
                Op::Print
            }
            "halt" => {
                require_end(&mut tokens, line_number)?;
                Op::Halt
            }
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        ops.push(op);
    }

    Ok(ops)
}
