fn pop_stack(stack: &mut Vec<i64>) -> Result<i64, String> {
    stack.pop().ok_or_else(|| "stack underflow".to_string())
}

fn pop_binary(stack: &mut Vec<i64>) -> Result<(i64, i64), String> {
    let b = pop_stack(stack)?;
    let a = pop_stack(stack)?;
    Ok((a, b))
}

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
                pc += 1;
            }
            Op::Pop => {
                pop_stack(&mut stack)?;
                pc += 1;
            }
            Op::Add => {
                let (a, b) = pop_binary(&mut stack)?;
                stack.push(a.wrapping_add(b));
                pc += 1;
            }
            Op::Sub => {
                let (a, b) = pop_binary(&mut stack)?;
                stack.push(a.wrapping_sub(b));
                pc += 1;
            }
            Op::Mul => {
                let (a, b) = pop_binary(&mut stack)?;
                stack.push(a.wrapping_mul(b));
                pc += 1;
            }
            Op::Div => {
                let (a, b) = pop_binary(&mut stack)?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                stack.push(a.wrapping_div(b));
                pc += 1;
            }
            Op::Mod => {
                let (a, b) = pop_binary(&mut stack)?;
                if b == 0 {
                    return Err("modulo by zero".to_string());
                }
                stack.push(a.wrapping_rem(b));
                pc += 1;
            }
            Op::Neg => {
                let value = pop_stack(&mut stack)?;
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
                let value = pop_stack(&mut stack)?;
                if value == 0 {
                    pc = *target;
                } else {
                    pc += 1;
                }
            }
            Op::Print => {
                output.push(pop_stack(&mut stack)?);
                pc += 1;
            }
            Op::Halt => return Ok(output),
        }
    }
}
