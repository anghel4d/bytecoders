fn pop_stack(stack: &mut Vec<i64>) -> Result<i64, String> {
    stack.pop().ok_or_else(|| "stack underflow".to_string())
}

pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0usize;

    loop {
        match program.get(pc) {
            None => return Err("program counter out of bounds".to_string()),
            Some(Op::Push(n)) => {
                stack.push(*n);
                pc = pc.wrapping_add(1);
            }
            Some(Op::Pop) => {
                pop_stack(&mut stack)?;
                pc = pc.wrapping_add(1);
            }
            Some(Op::Add) => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                stack.push(a.wrapping_add(b));
                pc = pc.wrapping_add(1);
            }
            Some(Op::Sub) => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                stack.push(a.wrapping_sub(b));
                pc = pc.wrapping_add(1);
            }
            Some(Op::Mul) => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                stack.push(a.wrapping_mul(b));
                pc = pc.wrapping_add(1);
            }
            Some(Op::Div) => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                stack.push(if a == i64::MIN && b == -1 {
                    i64::MIN
                } else {
                    a / b
                });
                pc = pc.wrapping_add(1);
            }
            Some(Op::Mod) => {
                let b = pop_stack(&mut stack)?;
                let a = pop_stack(&mut stack)?;
                if b == 0 {
                    return Err("modulo by zero".to_string());
                }
                stack.push(if a == i64::MIN && b == -1 {
                    0
                } else {
                    a % b
                });
                pc = pc.wrapping_add(1);
            }
            Some(Op::Neg) => {
                let a = pop_stack(&mut stack)?;
                stack.push(a.wrapping_neg());
                pc = pc.wrapping_add(1);
            }
            Some(Op::Dup) => {
                let a = stack
                    .last()
                    .copied()
                    .ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a);
                pc = pc.wrapping_add(1);
            }
            Some(Op::Swap) => {
                if stack.len() < 2 {
                    return Err("stack underflow".to_string());
                }
                let last = stack.len() - 1;
                stack.swap(last, last - 1);
                pc = pc.wrapping_add(1);
            }
            Some(Op::Jmp(target)) => {
                pc = *target;
            }
            Some(Op::Jz(target)) => {
                let a = pop_stack(&mut stack)?;
                if a == 0 {
                    pc = *target;
                } else {
                    pc = pc.wrapping_add(1);
                }
            }
            Some(Op::Print) => {
                output.push(pop_stack(&mut stack)?);
                pc = pc.wrapping_add(1);
            }
            Some(Op::Halt) => return Ok(output),
        }
    }
}
