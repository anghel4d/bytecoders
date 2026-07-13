pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        let op = match program.get(pc) {
            Some(op) => op,
            None => return Err("program counter out of bounds".to_string()),
        };

        match op {
            Op::Push(n) => {
                stack.push(*n);
                pc += 1;
            }
            Op::Pop => {
                stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                pc += 1;
            }
            Op::Add => {
                let b = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                let a = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_add(b));
                pc += 1;
            }
            Op::Sub => {
                let b = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                let a = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_sub(b));
                pc += 1;
            }
            Op::Mul => {
                let b = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                let a = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_mul(b));
                pc += 1;
            }
            Op::Div => {
                let b = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                let a = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                stack.push(if a == i64::MIN && b == -1 {
                    i64::MIN
                } else {
                    a / b
                });
                pc += 1;
            }
            Op::Mod => {
                let b = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                let a = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                if b == 0 {
                    return Err("modulo by zero".to_string());
                }
                stack.push(if a == i64::MIN && b == -1 {
                    0
                } else {
                    a % b
                });
                pc += 1;
            }
            Op::Neg => {
                let a = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_neg());
                pc += 1;
            }
            Op::Dup => {
                let a = stack
                    .last()
                    .copied()
                    .ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a);
                pc += 1;
            }
            Op::Swap => {
                if stack.len() < 2 {
                    return Err("stack underflow".to_string());
                }
                let top = stack.len() - 1;
                stack.swap(top, top - 1);
                pc += 1;
            }
            Op::Jmp(target) => {
                pc = *target;
            }
            Op::Jz(target) => {
                let a = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                if a == 0 {
                    pc = *target;
                } else {
                    pc += 1;
                }
            }
            Op::Print => {
                let a = stack
                    .pop()
                    .ok_or_else(|| "stack underflow".to_string())?;
                output.push(a);
                pc += 1;
            }
            Op::Halt => return Ok(output),
        }
    }
}
