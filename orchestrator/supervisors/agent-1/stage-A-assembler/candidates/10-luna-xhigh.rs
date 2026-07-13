fn assemble_is_label_name(name: &str) -> bool {
    let mut bytes = name.bytes();

    match bytes.next() {
        Some(b'a'..=b'z') | Some(b'A'..=b'Z') | Some(b'_') => {}
        _ => return false,
    }

    bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn assemble_error(line: usize, message: &str) -> String {
    format!("line {}: {}", line, message)
}

fn assemble_one_operand<'a>(tokens: &[&'a str]) -> Result<&'a str, &'static str> {
    if tokens.len() < 2 {
        Err("missing operand")
    } else if tokens.len() > 2 {
        Err("trailing tokens")
    } else {
        Ok(tokens[1])
    }
}

fn assemble_no_operand(tokens: &[&str]) -> Result<(), &'static str> {
    if tokens.len() == 1 {
        Ok(())
    } else {
        Err("trailing tokens")
    }
}

fn assemble_integer(token: &str) -> Result<i64, &'static str> {
    let digits = token.strip_prefix('-').unwrap_or(token);

    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("invalid operand");
    }

    token.parse::<i64>().map_err(|_| "invalid operand")
}

fn assemble_label_target<'a>(
    labels: &std::collections::HashMap<&'a str, usize>,
    name: &str,
) -> Result<usize, &'static str> {
    if !assemble_is_label_name(name) {
        return Err("invalid operand");
    }

    labels.get(name).copied().ok_or("undefined label")
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut lines = Vec::new();

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let code = raw_line.split(';').next().unwrap_or("");
        let tokens: Vec<&str> = code.split_whitespace().collect();

        if !tokens.is_empty() {
            lines.push((line_number, tokens));
        }
    }

    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0usize;

    for (line_number, tokens) in &lines {
        if tokens.len() == 1 && tokens[0].ends_with(':') {
            let name = &tokens[0][..tokens[0].len() - 1];

            if !assemble_is_label_name(name) {
                return Err(assemble_error(*line_number, "invalid label"));
            }

            if labels.insert(name, instruction_count).is_some() {
                return Err(assemble_error(*line_number, "duplicate label"));
            }
        } else {
            if tokens[0].ends_with(':') {
                return Err(assemble_error(*line_number, "trailing tokens"));
            }

            instruction_count += 1;
        }
    }

    let mut output = Vec::with_capacity(instruction_count);

    for (line_number, tokens) in lines {
        if tokens.len() == 1 && tokens[0].ends_with(':') {
            continue;
        }

        let op = match tokens[0] {
            "push" => {
                let operand = assemble_one_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                let value = assemble_integer(operand)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Push(value)
            }
            "pop" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Pop
            }
            "add" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Add
            }
            "sub" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Sub
            }
            "mul" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Mul
            }
            "div" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Div
            }
            "mod" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Mod
            }
            "neg" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Neg
            }
            "dup" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Dup
            }
            "swap" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Swap
            }
            "jmp" => {
                let operand = assemble_one_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                let target = assemble_label_target(&labels, operand)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Jmp(target)
            }
            "jz" => {
                let operand = assemble_one_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                let target = assemble_label_target(&labels, operand)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Jz(target)
            }
            "print" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Print
            }
            "halt" => {
                assemble_no_operand(&tokens)
                    .map_err(|message| assemble_error(line_number, message))?;
                Op::Halt
            }
            _ => return Err(assemble_error(line_number, "unknown mnemonic")),
        };

        output.push(op);
    }

    Ok(output)
}
