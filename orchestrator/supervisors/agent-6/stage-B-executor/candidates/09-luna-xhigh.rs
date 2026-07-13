fn pop_binary(stack: &mut Vec<i64>) -> Result<(i64, i64), String> {
    let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
    let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
    Ok((a, b))
}

pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0usize;

    loop {
        let op = program
            .get(pc)
            .ok_or_else(|| "program counter out of bounds".to_string())?;

        match op {
            Op::Push(value) => {
                stack.push(*value);
                pc = pc.wrapping_add(1);
            }
            Op::Pop => {
                stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                pc = pc.wrapping_add(1);
            }
            Op::Add => {
                let (a, b) = pop_binary(&mut stack)?;
                stack.push(a.wrapping_add(b));
                pc = pc.wrapping_add(1);
            }
            Op::Sub => {
                let (a, b) = pop_binary(&mut stack)?;
                stack.push(a.wrapping_sub(b));
                pc = pc.wrapping_add(1);
            }
            Op::Mul => {
                let (a, b) = pop_binary(&mut stack)?;
                stack.push(a.wrapping_mul(b));
                pc = pc.wrapping_add(1);
            }
            Op::Div => {
                let (a, b) = pop_binary(&mut stack)?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                stack.push(a.wrapping_div(b));
                pc = pc.wrapping_add(1);
            }
            Op::Mod => {
                let (a, b) = pop_binary(&mut stack)?;
                if b == 0 {
                    return Err("modulo by zero".to_string());
                }
                stack.push(a.wrapping_rem(b));
                pc = pc.wrapping_add(1);
            }
            Op::Neg => {
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_neg());
                pc = pc.wrapping_add(1);
            }
            Op::Dup => {
                let a = *stack.last().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a);
                pc = pc.wrapping_add(1);
            }
            Op::Swap => {
                if stack.len() < 2 {
                    return Err("stack underflow".to_string());
                }
                let last = stack.len() - 1;
                stack.swap(last, last - 1);
                pc = pc.wrapping_add(1);
            }
            Op::Jmp(target) => {
                pc = *target;
            }
            Op::Jz(target) => {
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                if a == 0 {
                    pc = *target;
                } else {
                    pc = pc.wrapping_add(1);
                }
            }
            Op::Print => {
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                output.push(a);
                pc = pc.wrapping_add(1);
            }
            Op::Halt => return Ok(output),
        }
    }
}
