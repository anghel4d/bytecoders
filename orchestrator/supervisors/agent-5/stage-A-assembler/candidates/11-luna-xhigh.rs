enum PendingOp {
    Push(i64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Dup,
    Swap,
    Jmp(String),
    Jz(String),
    Print,
    Halt,
}

enum ParsedLine {
    Label(String),
    Instruction(PendingOp),
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut parsed = Vec::new();
    let mut instruction_index = 0;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();

        let Some(entry) = parse_assembly_line(line)? else {
            continue;
        };

        match &entry {
            ParsedLine::Label(name) => {
                if labels.insert(name.clone(), instruction_index).is_some() {
                    return Err("duplicate label".to_string());
                }
            }
            ParsedLine::Instruction(_) => {
                instruction_index += 1;
            }
        }

        parsed.push(entry);
    }

    let mut output = Vec::with_capacity(instruction_index);

    for entry in parsed {
        match entry {
            ParsedLine::Label(_) => {}
            ParsedLine::Instruction(op) => {
                output.push(match op {
                    PendingOp::Push(value) => Op::Push(value),
                    PendingOp::Pop => Op::Pop,
                    PendingOp::Add => Op::Add,
                    PendingOp::Sub => Op::Sub,
                    PendingOp::Mul => Op::Mul,
                    PendingOp::Div => Op::Div,
                    PendingOp::Mod => Op::Mod,
                    PendingOp::Neg => Op::Neg,
                    PendingOp::Dup => Op::Dup,
                    PendingOp::Swap => Op::Swap,
                    PendingOp::Jmp(label) => {
                        let target = labels
                            .get(&label)
                            .copied()
                            .ok_or_else(|| "undefined label".to_string())?;
                        Op::Jmp(target)
                    }
                    PendingOp::Jz(label) => {
                        let target = labels
                            .get(&label)
                            .copied()
                            .ok_or_else(|| "undefined label".to_string())?;
                        Op::Jz(target)
                    }
                    PendingOp::Print => Op::Print,
                    PendingOp::Halt => Op::Halt,
                });
            }
        }
    }

    Ok(output)
}

fn parse_assembly_line(line: &str) -> Result<Option<ParsedLine>, String> {
    if line.is_empty() {
        return Ok(None);
    }

    let tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.len() == 1 && tokens[0].ends_with(':') {
        let name = &tokens[0][..tokens[0].len() - 1];

        if !is_label_name(name) {
            return Err("invalid label".to_string());
        }

        return Ok(Some(ParsedLine::Label(name.to_string())));
    }

    let mnemonic = tokens[0];

    let instruction = match mnemonic {
        "push" => {
            let operand = one_operand(&tokens)?;
            let value = operand
                .parse::<i64>()
                .map_err(|_| "invalid integer".to_string())?;
            PendingOp::Push(value)
        }
        "pop" => no_operand(&tokens, PendingOp::Pop)?,
        "add" => no_operand(&tokens, PendingOp::Add)?,
        "sub" => no_operand(&tokens, PendingOp::Sub)?,
        "mul" => no_operand(&tokens, PendingOp::Mul)?,
        "div" => no_operand(&tokens, PendingOp::Div)?,
        "mod" => no_operand(&tokens, PendingOp::Mod)?,
        "neg" => no_operand(&tokens, PendingOp::Neg)?,
        "dup" => no_operand(&tokens, PendingOp::Dup)?,
        "swap" => no_operand(&tokens, PendingOp::Swap)?,
        "jmp" => {
            let operand = one_operand(&tokens)?;

            if !is_label_name(operand) {
                return Err("invalid label".to_string());
            }

            PendingOp::Jmp(operand.to_string())
        }
        "jz" => {
            let operand = one_operand(&tokens)?;

            if !is_label_name(operand) {
                return Err("invalid label".to_string());
            }

            PendingOp::Jz(operand.to_string())
        }
        "print" => no_operand(&tokens, PendingOp::Print)?,
        "halt" => no_operand(&tokens, PendingOp::Halt)?,
        _ => return Err("unknown mnemonic".to_string()),
    };

    Ok(Some(ParsedLine::Instruction(instruction)))
}

fn one_operand<'a>(tokens: &'a [&'a str]) -> Result<&'a str, String> {
    match tokens.len() {
        1 => Err("missing operand".to_string()),
        2 => Ok(tokens[1]),
        _ => Err("trailing tokens".to_string()),
    }
}

fn no_operand(tokens: &[&str], op: PendingOp) -> Result<PendingOp, String> {
    if tokens.len() == 1 {
        Ok(op)
    } else {
        Err("trailing tokens".to_string())
    }
}

fn is_label_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
