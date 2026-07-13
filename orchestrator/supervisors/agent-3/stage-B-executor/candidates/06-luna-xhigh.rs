fn pop_operand(stack: &mut Vec<i64>) -> Result<i64, String> {
    stack.pop().ok_or_else(|| "stack underflow".to_string())
}

pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0usize;

    loop {
        let instruction = match program.get(pc) {
            Some(instruction) => instruction,
            None => return Err("program counter out of bounds".to_string()),
        };

        match instruction {
            Op::Push(value) => {
                stack.push(*value);
                pc = pc.wrapping_add(1);
            }
            Op::Pop => {
                pop_operand(&mut stack)?;
                pc = pc.wrapping_add(1);
            }
            Op::Add => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                stack.push(a.wrapping_add(b));
                pc = pc.wrapping_add(1);
            }
            Op::Sub => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                stack.push(a.wrapping_sub(b));
                pc = pc.wrapping_add(1);
            }
            Op::Mul => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                stack.push(a.wrapping_mul(b));
                pc = pc.wrapping_add(1);
            }
            Op::Div => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                stack.push(a.wrapping_div(b));
                pc = pc.wrapping_add(1);
            }
            Op::Mod => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                if b == 0 {
                    return Err("modulo by zero".to_string());
                }
                stack.push(a.wrapping_rem(b));
                pc = pc.wrapping_add(1);
            }
            Op::Neg => {
                let value = pop_operand(&mut stack)?;
                stack.push(value.wrapping_neg());
                pc = pc.wrapping_add(1);
            }
            Op::Dup => {
                let value = stack
                    .last()
                    .copied()
                    .ok_or_else(|| "stack underflow".to_string())?;
                stack.push(value);
                pc = pc.wrapping_add(1);
            }
            Op::Swap => {
                if stack.len() < 2 {
                    return Err("stack underflow".to_string());
                }
                let top = stack.len() - 1;
                stack.swap(top, top - 1);
                pc = pc.wrapping_add(1);
            }
            Op::Jmp(target) => {
                pc = *target;
            }
            Op::Jz(target) => {
                let value = pop_operand(&mut stack)?;
                if value == 0 {
                    pc = *target;
                } else {
                    pc = pc.wrapping_add(1);
                }
            }
            Op::Print => {
                output.push(pop_operand(&mut stack)?);
                pc = pc.wrapping_add(1);
            }
            Op::Halt => return Ok(output),
        }
    }
}
