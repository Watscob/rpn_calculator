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
        match c {
            '_' | ' ' | '\n' | '0' ..= '9' | '.' => self.handle_digit(c),
            '+' | '-' | '*' | '/' | '%' => self.handle_arith(c),
            'd' | 'r' | 'c' => self.handle_stack(c),
            'p' | 'f' => self.handle_print(c),
            'q' => return true,
            _ => panic!("'{}' is not implemented.", c),
        }

        return false
    }

    fn handle_digit(&mut self, c: char) {
        match c {
            '_' => self.buffer = format!("-{}", self.buffer),
            '0' ..= '9' | '.' => self.buffer.push(c),
            ' ' | '\n' => {
                if !self.buffer.is_empty() {
                    if let Ok(num) = self.buffer.parse::<f32>() {
                        self.stack.push(num);
                        self.buffer.clear();
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_arith(&mut self, c: char) {
        match c {
            '+' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 + op2);
                }
            }
            '-' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 - op2);
                }
            }
            '*' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 * op2);
                }
            }
            '/' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 / op2);
                }
            }
            '%' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1 % op2);
                }
            }
            _ => {}
        }
    }

    fn handle_print(&mut self, c: char) {
        match c {
            'p' => {
                println!("{}", self.stack.last().map_or("Stack is empty".to_string(), |&x| x.to_string()));
            }
            'n' => {
                print!("{}", self.stack.pop().map_or("Stack is empty".to_string(), |x| x.to_string()));
            }
            'f' => {
                if self.stack.is_empty() {
                    println!("Stack is empty");
                } else {
                    for e in self.stack.iter().rev() {
                        println!("{}", e);
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_stack(&mut self, c: char) {
        match c {
            'c' => self.stack.clear(),
            'd' => {
                if let Some(&top) = self.stack.last() {
                    self.stack.push(top);
                }
            }
            'r' => {
                if let (Some(op1), Some(op2)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(op1);
                    self.stack.push(op2);
                }
            }
            _ => {}
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
