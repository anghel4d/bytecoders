fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();

    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_'))
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instructions = Vec::new();
    let mut instruction_index = 0;

    for (line_index, line) in src.lines().enumerate() {
        let code = line.split_once(';').map_or(line, |(code, _)| code);
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if let Some(name) = tokens[0].strip_suffix(':') {
            if !valid_label(name) {
                return Err(format!("line {}: invalid label", line_index + 1));
            }
            if tokens.len() != 1 {
                return Err(format!("line {}: trailing tokens", line_index + 1));
            }
            if labels.insert(name, instruction_index).is_some() {
                return Err(format!("line {}: duplicate label", line_index + 1));
            }
            continue;
        }

        instructions.push((line_index + 1, tokens));
        instruction_index += 1;
    }

    let mut ops = Vec::with_capacity(instructions.len());

    for (line, tokens) in instructions {
        let no_operands = |op| {
            if tokens.len() == 1 {
                Ok(op)
            } else {
                Err(format!("line {}: trailing tokens", line))
            }
        };

        let op = match tokens[0] {
            "push" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line));
                }
                let value = tokens[1]
                    .parse::<i64>()
                    .map_err(|_| format!("line {}: invalid integer", line))?;
                Op::Push(value)
            }
            "pop" => no_operands(Op::Pop)?,
            "add" => no_operands(Op::Add)?,
            "sub" => no_operands(Op::Sub)?,
            "mul" => no_operands(Op::Mul)?,
            "div" => no_operands(Op::Div)?,
            "mod" => no_operands(Op::Mod)?,
            "neg" => no_operands(Op::Neg)?,
            "dup" => no_operands(Op::Dup)?,
            "swap" => no_operands(Op::Swap)?,
            "jmp" | "jz" => {
                if tokens.len() < 2 {
                    return Err(format!("line {}: missing operand", line));
                }
                if tokens.len() > 2 {
                    return Err(format!("line {}: trailing tokens", line));
                }
                if !valid_label(tokens[1]) {
                    return Err(format!("line {}: invalid label", line));
                }
                let target = *labels
                    .get(tokens[1])
                    .ok_or_else(|| format!("line {}: undefined label", line))?;
                if tokens[0] == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => no_operands(Op::Print)?,
            "halt" => no_operands(Op::Halt)?,
            _ => return Err(format!("line {}: unknown mnemonic", line)),
        };

        ops.push(op);
    }

    Ok(ops)
}
