fn pop_stack(stack: &mut Vec<i64>) -> Result<i64, String> {
    stack.pop().ok_or_else(|| "stack underflow".to_owned())
}

fn pop_two(stack: &mut Vec<i64>) -> Result<(i64, i64), String> {
    let b = pop_stack(stack)?;
    let a = pop_stack(stack)?;
    Ok((a, b))
}

pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        let op = program
            .get(pc)
            .ok_or_else(|| "program counter out of bounds".to_owned())?;

        match *op {
            Op::Push(n) => {
                stack.push(n);
                pc += 1;
            }
            Op::Pop => {
                pop_stack(&mut stack)?;
                pc += 1;
            }
            Op::Add => {
                let (a, b) = pop_two(&mut stack)?;
                stack.push(a.wrapping_add(b));
                pc += 1;
            }
            Op::Sub => {
                let (a, b) = pop_two(&mut stack)?;
                stack.push(a.wrapping_sub(b));
                pc += 1;
            }
            Op::Mul => {
                let (a, b) = pop_two(&mut stack)?;
                stack.push(a.wrapping_mul(b));
                pc += 1;
            }
            Op::Div => {
                let (a, b) = pop_two(&mut stack)?;
                if b == 0 {
                    return Err("division by zero".to_owned());
                }
                stack.push(a.wrapping_div(b));
                pc += 1;
            }
            Op::Mod => {
                let (a, b) = pop_two(&mut stack)?;
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
                let a = stack
                    .last()
                    .copied()
                    .ok_or_else(|| "stack underflow".to_owned())?;
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
                pc = target;
            }
            Op::Jz(target) => {
                let a = pop_stack(&mut stack)?;
                if a == 0 {
                    pc = target;
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
