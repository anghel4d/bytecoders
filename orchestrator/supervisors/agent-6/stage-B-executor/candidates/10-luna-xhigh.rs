fn pop_operand(stack: &mut Vec<i64>) -> Result<i64, String> {
    stack.pop().ok_or_else(|| "stack underflow".to_string())
}

fn binary_op<F>(stack: &mut Vec<i64>, op: F) -> Result<(), String>
where
    F: FnOnce(i64, i64) -> i64,
{
    let b = pop_operand(stack)?;
    let a = pop_operand(stack)?;
    stack.push(op(a, b));
    Ok(())
}

pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        match program.get(pc) {
            None => return Err("program counter out of bounds".to_string()),
            Some(&Op::Push(value)) => {
                stack.push(value);
                pc += 1;
            }
            Some(&Op::Pop) => {
                pop_operand(&mut stack)?;
                pc += 1;
            }
            Some(&Op::Add) => {
                binary_op(&mut stack, |a, b| a.wrapping_add(b))?;
                pc += 1;
            }
            Some(&Op::Sub) => {
                binary_op(&mut stack, |a, b| a.wrapping_sub(b))?;
                pc += 1;
            }
            Some(&Op::Mul) => {
                binary_op(&mut stack, |a, b| a.wrapping_mul(b))?;
                pc += 1;
            }
            Some(&Op::Div) => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                stack.push(a.wrapping_div(b));
                pc += 1;
            }
            Some(&Op::Mod) => {
                let b = pop_operand(&mut stack)?;
                let a = pop_operand(&mut stack)?;
                if b == 0 {
                    return Err("modulo by zero".to_string());
                }
                stack.push(a.wrapping_rem(b));
                pc += 1;
            }
            Some(&Op::Neg) => {
                let a = pop_operand(&mut stack)?;
                stack.push(a.wrapping_neg());
                pc += 1;
            }
            Some(&Op::Dup) => {
                let a = *stack.last().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a);
                pc += 1;
            }
            Some(&Op::Swap) => {
                if stack.len() < 2 {
                    return Err("stack underflow".to_string());
                }
                let len = stack.len();
                stack.swap(len - 1, len - 2);
                pc += 1;
            }
            Some(&Op::Jmp(target)) => {
                pc = target;
            }
            Some(&Op::Jz(target)) => {
                let a = pop_operand(&mut stack)?;
                if a == 0 {
                    pc = target;
                } else {
                    pc += 1;
                }
            }
            Some(&Op::Print) => {
                output.push(pop_operand(&mut stack)?);
                pc += 1;
            }
            Some(&Op::Halt) => return Ok(output),
        }
    }
}
