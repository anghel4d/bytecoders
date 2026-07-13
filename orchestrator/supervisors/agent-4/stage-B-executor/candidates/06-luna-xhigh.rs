pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        let op = match program.get(pc) {
            Some(op) => op,
            None => return Err(String::from("program counter out of bounds")),
        };

        match op {
            Op::Push(value) => {
                stack.push(*value);
                pc += 1;
            }
            Op::Pop => {
                if stack.pop().is_none() {
                    return Err(String::from("stack underflow"));
                }
                pc += 1;
            }
            Op::Add => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                stack.push(a.wrapping_add(b));
                pc += 1;
            }
            Op::Sub => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                stack.push(a.wrapping_sub(b));
                pc += 1;
            }
            Op::Mul => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                stack.push(a.wrapping_mul(b));
                pc += 1;
            }
            Op::Div => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                if b == 0 {
                    return Err(String::from("division by zero"));
                }
                stack.push(a.wrapping_div(b));
                pc += 1;
            }
            Op::Mod => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                if b == 0 {
                    return Err(String::from("modulo by zero"));
                }
                stack.push(a.wrapping_rem(b));
                pc += 1;
            }
            Op::Neg => {
                let value = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                stack.push(value.wrapping_neg());
                pc += 1;
            }
            Op::Dup => {
                let value = match stack.last() {
                    Some(value) => *value,
                    None => return Err(String::from("stack underflow")),
                };
                stack.push(value);
                pc += 1;
            }
            Op::Swap => {
                if stack.len() < 2 {
                    return Err(String::from("stack underflow"));
                }
                let last = stack.len() - 1;
                stack.swap(last, last - 1);
                pc += 1;
            }
            Op::Jmp(target) => {
                pc = *target;
            }
            Op::Jz(target) => {
                let value = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                if value == 0 {
                    pc = *target;
                } else {
                    pc += 1;
                }
            }
            Op::Print => {
                let value = match stack.pop() {
                    Some(value) => value,
                    None => return Err(String::from("stack underflow")),
                };
                output.push(value);
                pc += 1;
            }
            Op::Halt => return Ok(output),
        }
    }
}
