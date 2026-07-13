fn pop_stack(stack: &mut Vec<i64>) -> Result<i64, String> {
    stack.pop().ok_or_else(|| "stack underflow".to_owned())
}

pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        let instruction = match program.get(pc) {
            Some(instruction) => instruction,
            None => return Err("program counter out of bounds".to_owned()),
        };

        match instruction {
            Op::Push(value) => {
                stack.push(*value);
                pc += 1;
            }
            Op::Pop => {
                pop_stack(&mut stack)?;
                pc += 1;
            }
            Op::Add => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                stack.push(a.wrapping_add(b));
                pc += 1;
            }
            Op::Sub => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                stack.push(a.wrapping_sub(b));
                pc += 1;
            }
            Op::Mul => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                stack.push(a.wrapping_mul(b));
                pc += 1;
            }
            Op::Div => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                if b == 0 {
                    return Err("division by zero".to_owned());
                }
                stack.push(a.wrapping_div(b));
                pc += 1;
            }
            Op::Mod => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                if b == 0 {
                    return Err("modulo by zero".to_owned());
                }
                stack.push(a.wrapping_rem(b));
                pc += 1;
            }
            Op::Neg => {
                let a = pop_stack(&mut stack)?;
                stack.push(a.wrapping_neg());
                pc += 1;
            }
            Op::Dup => {
                let a = match stack.last() {
                    Some(value) => *value,
                    None => return Err("stack underflow".to_owned()),
                };
                stack.push(a);
                pc += 1;
            }
            Op::Swap => {
                if stack.len() < 2 {
                    return Err("stack underflow".to_owned());
                }
                let last = stack.len() - 1;
                stack.swap(last, last - 1);
                pc += 1;
            }
            Op::Jmp(target) => {
                pc = *target;
            }
            Op::Jz(target) => {
                let a = pop_stack(&mut stack)?;
                if a == 0 {
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
