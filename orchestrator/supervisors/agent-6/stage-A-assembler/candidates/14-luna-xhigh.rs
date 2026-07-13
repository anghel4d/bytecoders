enum AsmInstr {
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

enum AsmLine {
    Label(String),
    Instr(AsmInstr),
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut lines = Vec::new();
    let mut instruction_count = 0usize;

    for line in src.lines() {
        if let Some(parsed) = parse_asm_line(line)? {
            match &parsed {
                AsmLine::Label(name) => {
                    if labels.insert(name.clone(), instruction_count).is_some() {
                        return Err("duplicate label".to_string());
                    }
                }
                AsmLine::Instr(_) => {
                    instruction_count += 1;
                }
            }
            lines.push(parsed);
        }
    }

    let mut output = Vec::with_capacity(instruction_count);

    for line in lines {
        match line {
            AsmLine::Label(_) => {}
            AsmLine::Instr(instr) => match instr {
                AsmInstr::Push(value) => output.push(Op::Push(value)),
                AsmInstr::Pop => output.push(Op::Pop),
                AsmInstr::Add => output.push(Op::Add),
                AsmInstr::Sub => output.push(Op::Sub),
                AsmInstr::Mul => output.push(Op::Mul),
                AsmInstr::Div => output.push(Op::Div),
                AsmInstr::Mod => output.push(Op::Mod),
                AsmInstr::Neg => output.push(Op::Neg),
                AsmInstr::Dup => output.push(Op::Dup),
                AsmInstr::Swap => output.push(Op::Swap),
                AsmInstr::Jmp(name) => {
                    let target = labels
                        .get(&name)
                        .copied()
                        .ok_or_else(|| "undefined label".to_string())?;
                    output.push(Op::Jmp(target));
                }
                AsmInstr::Jz(name) => {
                    let target = labels
                        .get(&name)
                        .copied()
                        .ok_or_else(|| "undefined label".to_string())?;
                    output.push(Op::Jz(target));
                }
                AsmInstr::Print => output.push(Op::Print),
                AsmInstr::Halt => output.push(Op::Halt),
            },
        }
    }

    Ok(output)
}

fn parse_asm_line(line: &str) -> Result<Option<AsmLine>, String> {
    let code = line.split(';').next().unwrap_or("");
    let tokens: Vec<&str> = code.split_whitespace().collect();

    if tokens.is_empty() {
        return Ok(None);
    }

    if tokens.len() == 1 && tokens[0].ends_with(':') {
        let name = &tokens[0][..tokens[0].len() - 1];
        if !is_label_name(name) {
            return Err("invalid label".to_string());
        }
        return Ok(Some(AsmLine::Label(name.to_string())));
    }

    if tokens[0].ends_with(':') {
        return Err("trailing tokens".to_string());
    }

    let mnemonic = tokens[0];
    let instr = match mnemonic {
        "push" => {
            if tokens.len() < 2 {
                return Err("missing operand".to_string());
            }
            if tokens.len() > 2 {
                return Err("trailing tokens".to_string());
            }
            let value = tokens[1]
                .parse::<i64>()
                .map_err(|_| "invalid operand".to_string())?;
            AsmInstr::Push(value)
        }
        "pop" => parse_no_operand(&tokens, AsmInstr::Pop)?,
        "add" => parse_no_operand(&tokens, AsmInstr::Add)?,
        "sub" => parse_no_operand(&tokens, AsmInstr::Sub)?,
        "mul" => parse_no_operand(&tokens, AsmInstr::Mul)?,
        "div" => parse_no_operand(&tokens, AsmInstr::Div)?,
        "mod" => parse_no_operand(&tokens, AsmInstr::Mod)?,
        "neg" => parse_no_operand(&tokens, AsmInstr::Neg)?,
        "dup" => parse_no_operand(&tokens, AsmInstr::Dup)?,
        "swap" => parse_no_operand(&tokens, AsmInstr::Swap)?,
        "print" => parse_no_operand(&tokens, AsmInstr::Print)?,
        "halt" => parse_no_operand(&tokens, AsmInstr::Halt)?,
        "jmp" => parse_jump(&tokens, false)?,
        "jz" => parse_jump(&tokens, true)?,
        _ => return Err("unknown mnemonic".to_string()),
    };

    Ok(Some(AsmLine::Instr(instr)))
}

fn parse_no_operand(tokens: &[&str], instr: AsmInstr) -> Result<AsmInstr, String> {
    if tokens.len() != 1 {
        return Err("trailing tokens".to_string());
    }
    Ok(instr)
}

fn parse_jump(tokens: &[&str], conditional: bool) -> Result<AsmInstr, String> {
    if tokens.len() < 2 {
        return Err("missing operand".to_string());
    }
    if tokens.len() > 2 {
        return Err("trailing tokens".to_string());
    }
    if !is_label_name(tokens[1]) {
        return Err("invalid operand".to_string());
    }

    if conditional {
        Ok(AsmInstr::Jz(tokens[1].to_string()))
    } else {
        Ok(AsmInstr::Jmp(tokens[1].to_string()))
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
