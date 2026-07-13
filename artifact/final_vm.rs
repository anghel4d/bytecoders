// ============================================================================
// Stack-machine bytecode VM — result of a 3-generation evolutionary contest.
// 6 blind Opus supervisor agents; each commissioned 15 claudex implementations
// per component (90/stage, 270 total). Winners, selected by objective fitness on
// a 16-program battery, propagated as fixed substrate to the next generation:
//   Component A  assembler  (agent 2, gpt-5.6-sol / high,  93L)
//   Component B  executor   (agent 6, gpt-5.6-sol / high,  91L)
//   Component C  optimizer  (agent 2, gpt-5.6-sol / high, 117L, basic-block fold)
// Assembled pipeline: 16/16 battery, 23 instructions folded away.
// ============================================================================

// Fixed ISA shared by all candidates. Concatenated at crate root; never redefined by candidates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Push(i64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Dup,
    Swap,
    Jmp(usize),
    Jz(usize),
    Print,
    Halt,
}

pub mod assembler { #[allow(unused_imports)] use super::Op::{self, *};
fn valid_label(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some('A'..='Z' | 'a'..='z' | '_'))
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn assemble(src: &str) -> Result<Vec<Op>, String> {
    let mut labels = std::collections::HashMap::new();
    let mut instruction_count = 0;

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }

        if let Some(name) = line.strip_suffix(':') {
            if !valid_label(name) {
                return Err(format!("line {line_number}: invalid label"));
            }
            if labels.insert(name, instruction_count).is_some() {
                return Err(format!("line {line_number}: duplicate label"));
            }
        } else {
            instruction_count += 1;
        }
    }

    let mut ops = Vec::with_capacity(instruction_count);

    for (line_index, raw_line) in src.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.split(';').next().unwrap_or("").trim();

        if line.is_empty() || line.ends_with(':') {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let mnemonic = tokens.next().unwrap();

        let op = match mnemonic {
            "push" => {
                let operand = tokens
                    .next()
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?;
                let value = operand
                    .parse::<i64>()
                    .map_err(|_| format!("line {line_number}: invalid operand"))?;
                Op::Push(value)
            }
            "pop" => Op::Pop,
            "add" => Op::Add,
            "sub" => Op::Sub,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "neg" => Op::Neg,
            "dup" => Op::Dup,
            "swap" => Op::Swap,
            "jmp" | "jz" => {
                let operand = tokens
                    .next()
                    .ok_or_else(|| format!("line {line_number}: missing operand"))?;
                if !valid_label(operand) {
                    return Err(format!("line {line_number}: invalid operand"));
                }
                let target = labels
                    .get(operand)
                    .copied()
                    .ok_or_else(|| format!("line {line_number}: undefined label"))?;
                if mnemonic == "jmp" {
                    Op::Jmp(target)
                } else {
                    Op::Jz(target)
                }
            }
            "print" => Op::Print,
            "halt" => Op::Halt,
            _ => return Err(format!("line {line_number}: unknown mnemonic")),
        };

        if tokens.next().is_some() {
            return Err(format!("line {line_number}: trailing tokens"));
        }

        ops.push(op);
    }

    Ok(ops)
}
}

pub mod optimizer { #[allow(unused_imports)] use super::Op::{self, *};
fn copy_op(op: &Op) -> Op {
    match op {
        Op::Push(value) => Op::Push(*value),
        Op::Pop => Op::Pop,
        Op::Add => Op::Add,
        Op::Sub => Op::Sub,
        Op::Mul => Op::Mul,
        Op::Div => Op::Div,
        Op::Mod => Op::Mod,
        Op::Neg => Op::Neg,
        Op::Dup => Op::Dup,
        Op::Swap => Op::Swap,
        Op::Jmp(target) => Op::Jmp(*target),
        Op::Jz(target) => Op::Jz(*target),
        Op::Print => Op::Print,
        Op::Halt => Op::Halt,
    }
}

fn reduce_tail(code: &mut Vec<Op>) {
    loop {
        let len = code.len();

        if len >= 2 {
            if matches!((&code[len - 2], &code[len - 1]), (Op::Push(_), Op::Pop)) {
                code.truncate(len - 2);
                continue;
            }

            if let (Op::Push(value), Op::Neg) = (&code[len - 2], &code[len - 1]) {
                let value = value.wrapping_neg();
                code.truncate(len - 2);
                code.push(Op::Push(value));
                continue;
            }
        }

        if len >= 3 {
            let folded = match (&code[len - 3], &code[len - 2], &code[len - 1]) {
                (Op::Push(a), Op::Push(b), Op::Add) => Some(a.wrapping_add(*b)),
                (Op::Push(a), Op::Push(b), Op::Sub) => Some(a.wrapping_sub(*b)),
                (Op::Push(a), Op::Push(b), Op::Mul) => Some(a.wrapping_mul(*b)),
                (Op::Push(a), Op::Push(b), Op::Div) if *b != 0 => {
                    Some(a.wrapping_div(*b))
                }
                (Op::Push(a), Op::Push(b), Op::Mod) if *b != 0 => {
                    Some(a.wrapping_rem(*b))
                }
                _ => None,
            };

            if let Some(value) = folded {
                code.truncate(len - 3);
                code.push(Op::Push(value));
                continue;
            }
        }

        break;
    }
}

pub fn optimize(program: &[Op]) -> Vec<Op> {
    let program_len = program.len();
    let mut boundaries = vec![false; program_len + 1];
    boundaries[0] = true;
    boundaries[program_len] = true;

    for op in program {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => *target,
            _ => continue,
        };

        if target <= program_len {
            boundaries[target] = true;
        }
    }

    let mut old_to_new = vec![usize::MAX; program_len + 1];
    let mut optimized = Vec::with_capacity(program_len);
    let mut start = 0;

    while start < program_len {
        old_to_new[start] = optimized.len();

        let mut end = start + 1;
        while !boundaries[end] {
            end += 1;
        }

        let mut segment = Vec::with_capacity(end - start);
        for op in &program[start..end] {
            segment.push(copy_op(op));
            reduce_tail(&mut segment);
        }
        optimized.extend(segment);

        start = end;
    }

    old_to_new[program_len] = optimized.len();

    for op in &mut optimized {
        let target = match op {
            Op::Jmp(target) | Op::Jz(target) => target,
            _ => continue,
        };

        let old_target = *target;
        if old_target <= program_len {
            *target = old_to_new[old_target];
        }
    }

    optimized
}
}

pub mod vm { #[allow(unused_imports)] use super::Op::{self, *};
pub fn run(program: &[Op]) -> Result<Vec<i64>, String> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        let op = program.get(pc).ok_or_else(|| "program counter out of bounds".to_string())?;

        match op {
            Op::Push(n) => {
                stack.push(*n);
                pc += 1;
            }
            Op::Pop => {
                stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                pc += 1;
            }
            Op::Add => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_add(b));
                pc += 1;
            }
            Op::Sub => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_sub(b));
                pc += 1;
            }
            Op::Mul => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_mul(b));
                pc += 1;
            }
            Op::Div => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                if b == 0 {
                    return Err("division by zero".to_string());
                }
                stack.push(a.wrapping_div(b));
                pc += 1;
            }
            Op::Mod => {
                let b = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                if b == 0 {
                    return Err("modulo by zero".to_string());
                }
                stack.push(a.wrapping_rem(b));
                pc += 1;
            }
            Op::Neg => {
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a.wrapping_neg());
                pc += 1;
            }
            Op::Dup => {
                let a = *stack.last().ok_or_else(|| "stack underflow".to_string())?;
                stack.push(a);
                pc += 1;
            }
            Op::Swap => {
                let len = stack.len();
                if len < 2 {
                    return Err("stack underflow".to_string());
                }
                stack.swap(len - 1, len - 2);
                pc += 1;
            }
            Op::Jmp(target) => {
                pc = *target;
            }
            Op::Jz(target) => {
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                if a == 0 {
                    pc = *target;
                } else {
                    pc += 1;
                }
            }
            Op::Print => {
                let a = stack.pop().ok_or_else(|| "stack underflow".to_string())?;
                output.push(a);
                pc += 1;
            }
            Op::Halt => return Ok(output),
        }
    }
}
}

/// assemble -> optimize -> run.
pub fn execute(src: &str) -> Result<Vec<i64>, String> {
    let program = assembler::assemble(src)?;
    let optimized = optimizer::optimize(&program);
    vm::run(&optimized)
}

fn main() {
    let src = "\
        push 2\n push 3\n add\n push 4\n mul\n print\n\
        push 3\n\
        loop:\n dup\n jz end\n dup\n print\n push 1\n sub\n jmp loop\n\
        end:\n halt\n";
    match assembler::assemble(src) {
        Ok(prog) => {
            let opt = optimizer::optimize(&prog);
            println!("assembled : {} instructions", prog.len());
            println!("optimized : {} instructions ({} folded away)", opt.len(), prog.len() - opt.len());
            match vm::run(&opt) {
                Ok(out) => println!("output    : {:?}", out),
                Err(e) => println!("runtime error: {}", e),
            }
        }
        Err(e) => println!("assembly error: {}", e),
    }
}
