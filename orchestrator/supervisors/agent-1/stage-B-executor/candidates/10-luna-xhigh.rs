pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        let op = match program.get(pc) {
            Some(op) => op,
            None => return Err("program counter out of bounds".to_string()),
        };

        match op {
            Op::Push(n) => {
                stack.push(*n);
                pc += 1;
            }
            Op::Pop => {
                if stack.pop().is_none() {
                    return Err("stack underflow".to_string());
                }
                pc += 1;
            }
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => return Err("stack underflow".to_string()),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => return Err("stack underflow".to_string()),
                };

                let value = match op {
                    Op::Add => a.wrapping_add(b),
                    Op::Sub => a.wrapping_sub(b),
                    Op::Mul => a.wrapping_mul(b),
                    Op::Div => {
                        if b == 0 {
                            return Err("division by zero".to_string());
                        }
                        a.wrapping_div(b)
                    }
                    Op::Mod => {
                        if b == 0 {
                            return Err("modulo by zero".to_string());
                        }
                        a.wrapping_rem(b)
                    }
                    _ => unreachable!(),
                };

                stack.push(value);
                pc += 1;
            }
            Op::Neg => {
                let a = match stack.pop() {
                    Some(value) => value,
                    None => return Err("stack underflow".to_string()),
                };
                stack.push(a.wrapping_neg());
                pc += 1;
            }
            Op::Dup => {
                let a = match stack.last() {
                    Some(&value) => value,
                    None => return Err("stack underflow".to_string()),
                };
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
                let a = match stack.pop() {
                    Some(value) => value,
                    None => return Err("stack underflow".to_string()),
                };
                if a == 0 {
                    pc = *target;
                } else {
                    pc += 1;
                }
            }
            Op::Print => {
                let a = match stack.pop() {
                    Some(value) => value,
                    None => return Err("stack underflow".to_string()),
                };
                output.push(a);
                pc += 1;
            }
            Op::Halt => return Ok(output),
        }
    }
}
