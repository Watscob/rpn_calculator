use std::io::{self, Read};

struct RPN {
    buffer: String,
    stack: Vec<f32>,
}

impl RPN {
    pub fn new() -> Self {
        RPN {
            buffer: String::new(),
            stack: Vec::new(),
        }
    }

    pub fn handle_byte(&mut self, c: char) -> bool {
        let _ = self.handle_digit(c)
             || self.handle_arith(c)
             || self.handle_stack(c)
             || self.handle_print(c);

        c == 'q'
    }

    fn handle_digit(&mut self, c: char) -> bool{
        match c {
            '_' => {
                self.buffer = format!("-{}", self.buffer);
                true
            }
            '0' ..= '9' | '.' => {
                self.buffer.push(c);
                true
            }
            ' ' | '\n' => {
                if !self.buffer.is_empty() {
                    if let Ok(num) = self.buffer.parse::<f32>() {
                        self.stack.push(num);
                        self.buffer.clear();
                    }
                }
                true
            }
            _ => false
        }
    }

    fn handle_arith(&mut self, c: char) -> bool {
        match c {
            '+' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 + op2);
                }
                true
            }
            '-' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 - op2);
                }
                true
            }
            '*' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 * op2);
                }
                true
            }
            '/' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 / op2);
                }
                true
            }
            '%' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 % op2);
                }
                true
            }
            _ => false
        }
    }

    fn handle_print(&mut self, c: char) -> bool {
        match c {
            'p' => {
                println!("{}", self.stack.last().map_or("Stack is empty".to_string(), |&x| x.to_string()));
                true
            }
            'n' => {
                print!("{}", self.stack.pop().map_or("Stack is empty".to_string(), |x| x.to_string()));
                true
            }
            'f' => {
                if self.stack.is_empty() {
                    println!("Stack is empty");
                } else {
                    for e in self.stack.iter().rev() {
                        println!("{}", e);
                    }
                }
                true
            }
            _ => false
        }
    }

    fn handle_stack(&mut self, c: char) -> bool {
        match c {
            'c' => {
                self.stack.clear();
                true
            }
            'd' => {
                if let Some(&top) = self.stack.last() {
                    self.stack.push(top);
                }
                true
            }
            'r' => {
                if let (Some(op1), Some(op2)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1);
                    self.stack.push(op2);
                }
                true
            }
            _ => false
        }
    }
}

fn main() {
    let stdio = io::stdin();
    let mut rpn = RPN::new();

    for byte in stdio.lock().bytes() {
        let quit = match byte {
            Ok(b) => rpn.handle_byte(b as char),
            Err(_) => false,
        };

        if quit {
            return;
        }
    }
}
