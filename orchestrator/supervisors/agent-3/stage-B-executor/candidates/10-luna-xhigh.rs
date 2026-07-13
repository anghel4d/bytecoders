fn pop_operand(stack: &mut Vec<i64>) -> Result<i64, String> {
    stack.pop().ok_or_else(|| "stack underflow".to_owned())
}

fn pop_operands(stack: &mut Vec<i64>) -> Result<(i64, i64), String> {
    let b = pop_operand(stack)?;
    let a = pop_operand(stack)?;
    Ok((a, b))
}

pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0usize;

    loop {
        if pc >= program.len() {
            return Err("program counter out of bounds".to_owned());
        }

        match &program[pc] {
            Op::Push(value) => {
                stack.push(*value);
                pc += 1;
            }
            Op::Pop => {
                pop_operand(&mut stack)?;
                pc += 1;
            }
            Op::Add => {
                let (a, b) = pop_operands(&mut stack)?;
                stack.push(a.wrapping_add(b));
                pc += 1;
            }
            Op::Sub => {
                let (a, b) = pop_operands(&mut stack)?;
                stack.push(a.wrapping_sub(b));
                pc += 1;
            }
            Op::Mul => {
                let (a, b) = pop_operands(&mut stack)?;
                stack.push(a.wrapping_mul(b));
                pc += 1;
            }
            Op::Div => {
                let (a, b) = pop_operands(&mut stack)?;
                if b == 0 {
                    return Err("division by zero".to_owned());
                }
                stack.push(a.wrapping_div(b));
                pc += 1;
            }
            Op::Mod => {
                let (a, b) = pop_operands(&mut stack)?;
                if b == 0 {
                    return Err("modulo by zero".to_owned());
                }
                stack.push(a.wrapping_rem(b));
                pc += 1;
            }
            Op::Neg => {
                let value = pop_operand(&mut stack)?;
                stack.push(value.wrapping_neg());
                pc += 1;
            }
            Op::Dup => {
                let value = *stack.last().ok_or_else(|| "stack underflow".to_owned())?;
                stack.push(value);
                pc += 1;
            }
            Op::Swap => {
                if stack.len() < 2 {
                    return Err("stack underflow".to_owned());
                }
                let len = stack.len();
                stack.swap(len - 1, len - 2);
                pc += 1;
            }
            Op::Jmp(target) => {
                pc = *target;
            }
            Op::Jz(target) => {
                let value = pop_operand(&mut stack)?;
                if value == 0 {
                    pc = *target;
                } else {
                    pc += 1;
                }
            }
            Op::Print => {
                output.push(pop_operand(&mut stack)?);
                pc += 1;
            }
            Op::Halt => return Ok(output),
        }
    }
}
