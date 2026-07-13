pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        if pc >= program.len() {
            return Err("program counter out of bounds".to_string());
        }

        match &program[pc] {
            Op::Push(value) => {
                stack.push(*value);
                pc = pc.wrapping_add(1);
            }
            Op::Pop => {
                if stack.pop().is_none() {
                    return Err("stack underflow".to_string());
                }
                pc = pc.wrapping_add(1);
            }
            Op::Add => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_add(b));
                pc = pc.wrapping_add(1);
            }
            Op::Sub => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_sub(b));
                pc = pc.wrapping_add(1);
            }
            Op::Mul => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_mul(b));
                pc = pc.wrapping_add(1);
            }
            Op::Div => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                stack.push(a.wrapping_div(b));
                pc = pc.wrapping_add(1);
            }
            Op::Mod => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                if b == 0 {
                    return Err("modulo by zero".to_string());
                }
                stack.push(a.wrapping_rem(b));
                pc = pc.wrapping_add(1);
            }
            Op::Neg => {
                let value = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(value.wrapping_neg());
                pc = pc.wrapping_add(1);
            }
            Op::Dup => {
                let value = *stack.last().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(value);
                pc = pc.wrapping_add(1);
            }
            Op::Swap => {
                if stack.len() < 2 {
                    return Err("stack underflow".to_string());
                }
                let len = stack.len();
                stack.swap(len - 1, len - 2);
                pc = pc.wrapping_add(1);
            }
            Op::Jmp(target) => {
                pc = *target;
            }
            Op::Jz(target) => {
                let value = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                if value == 0 {
                    pc = *target;
                } else {
                    pc = pc.wrapping_add(1);
                }
            }
            Op::Print => {
                let value = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                output.push(value);
                pc = pc.wrapping_add(1);
            }
            Op::Halt => return Ok(output),
        }
    }
}
