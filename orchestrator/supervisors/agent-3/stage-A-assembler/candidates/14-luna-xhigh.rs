fn assembly_label_name<'a>(tokens: &[&'a str]) -> Option<&'a str> {
    if tokens.len() == 1 && tokens[0].ends_with(':') {
        Some(&tokens[0][..tokens[0].len() - 1])
    } else {
        None
    }
}

fn valid_assembly_label(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn expect_assembly_arity(tokens: &[&str], expected: usize) -> Result<(), String> {
    if tokens.len() < expected {
        Err("missing operand".to_string())
    } else if tokens.len() > expected {
        Err("trailing tokens".to_string())
    } else {
        Ok(())
    }
}

fn validate_assembly_tokens(tokens: &[&str]) -> Result<(), String> {
    match tokens[0] {
        "push" => {
            expect_assembly_arity(tokens, 2)?;
            tokens[1]
                .parse::<i64>()
                .map_err(|_| "invalid operand".to_string())?;
            Ok(())
        }
        "jmp" | "jz" => {
            expect_assembly_arity(tokens, 2)?;
            if valid_assembly_label(tokens[1]) {
                Ok(())
            } else {
                Err("invalid operand".to_string())
            }
        }
        "pop" | "add" | "sub" | "mul" | "div" | "mod" | "neg" | "dup" | "swap"
        | "print" | "halt" => expect_assembly_arity(tokens, 1),
        _ => Err("unknown mnemonic".to_string()),
    }
}

fn assemble_instruction(
    tokens: &[&str],
    labels: &std::collections::HashMap<&str, usize>,
) -> Result<Op, String> {
    match tokens[0] {
        "push" => Ok(Op::Push(
            tokens[1]
                .parse::<i64>()
                .map_err(|_| "invalid operand".to_string())?,
        )),
        "pop" => Ok(Op::Pop),
        "add" => Ok(Op::Add),
        "sub" => Ok(Op::Sub),
        "mul" => Ok(Op::Mul),
        "div" => Ok(Op::Div),
        "mod" => Ok(Op::Mod),
        "neg" => Ok(Op::Neg),
        "dup" => Ok(Op::Dup),
        "swap" => Ok(Op::Swap),
        "jmp" => Ok(Op::Jmp(
            labels
                .get(tokens[1])
                .copied()
                .ok_or_else(|| "undefined label".to_string())?,
        )),
        "jz" => Ok(Op::Jz(
            labels
                .get(tokens[1])
                .copied()
                .ok_or_else(|| "undefined label".to_string())?,
        )),
        "print" => Ok(Op::Print),
        "halt" => Ok(Op::Halt),
        _ => Err("unknown mnemonic".to_string()),
    }
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut lines = Vec::new();
    let mut instruction_count = 0;

    for raw_line in src.lines() {
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if let Some(label) = assembly_label_name(&tokens) {
            if !valid_assembly_label(label) {
                return Err("invalid label".to_string());
            }
            if labels.insert(label, instruction_count).is_some() {
                return Err("duplicate label".to_string());
            }
        } else {
            validate_assembly_tokens(&tokens)?;
            instruction_count += 1;
        }

        lines.push(tokens);
    }

    let mut result = Vec::with_capacity(instruction_count);

    for tokens in lines {
        if assembly_label_name(&tokens).is_some() {
            continue;
        }
        result.push(assemble_instruction(&tokens, &labels)?);
    }

    Ok(result)
}
