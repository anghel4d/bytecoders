fn pop_operand(stack: &mut Vec<i64>) -> Result<i64, String> {
    stack.pop().ok_or_else(|| "stack underflow".to_string())
}

pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0usize;

    loop {
        if pc >= program.len() {
            return Err("program counter out of bounds".to_string());
        }

        match &program[pc] {
            Op::Push(n) => {
                stack.push(*n);
                pc += 1;
            }
            Op::Pop => {
                pop_operand(&mut stack)?;
                pc += 1;
            }
            Op::Add => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                stack.push(a.wrapping_add(b));
                pc += 1;
            }
            Op::Sub => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                stack.push(a.wrapping_sub(b));
                pc += 1;
            }
            Op::Mul => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                stack.push(a.wrapping_mul(b));
                pc += 1;
            }
            Op::Div => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
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
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
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
                let a = pop_operand(&mut stack)?;
                stack.push(a.wrapping_neg());
                pc += 1;
            }
            Op::Dup => {
                let a = *stack.last().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a);
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
                let a = pop_operand(&mut stack)?;
                if a == 0 {
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
