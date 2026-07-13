pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        let instruction = match program.get(pc) {
            Some(instruction) => instruction,
            None => return Err("program counter out of bounds".to_string()),
        };

        match instruction {
            Op::Push(value) => {
                stack.push(*value);
                pc += 1;
            }
            Op::Pop => {
                stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                pc += 1;
            }
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod => {
                let b = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                let a = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;

                let value = match instruction {
                    Op::Add => a.wrapping_add(b),
                    Op::Sub => a.wrapping_sub(b),
                    Op::Mul => a.wrapping_mul(b),
                    Op::Div => {
                        if b == 0 {
                            return Err("division by zero".to_string());
                        }
                        a.wrapping_div(b)
                    }
                    Op::Mod => {
                        if b == 0 {
                            return Err("modulo by zero".to_string());
                        }
                        a.wrapping_rem(b)
                    }
                    _ => unreachable!(),
                };

                stack.push(value);
                pc += 1;
            }
            Op::Neg => {
                let value = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                stack.push(value.wrapping_neg());
                pc += 1;
            }
            Op::Dup => {
                let value = stack
                    .last()
                    .copied()
                    .ok_or_else(|| "stack underflow".to_string())?;
                stack.push(value);
                pc += 1;
            }
            Op::Swap => {
                if stack.len() < 2 {
                    return Err("stack underflow".to_string());
                }
                let len = stack.len();
                stack.swap(len - 1, len - 2);
                pc += 1;
            }
            Op::Jmp(target) => {
                pc = *target;
            }
            Op::Jz(target) => {
                let value = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                if value == 0 {
                    pc = *target;
                } else {
                    pc += 1;
                }
            }
            Op::Print => {
                let value = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                output.push(value);
                pc += 1;
            }
            Op::Halt => return Ok(output),
        }
    }
}
