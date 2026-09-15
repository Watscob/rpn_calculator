use std::io::{self, Read};

struct RPN {
    number: Option<(isize, bool)>,
    stack: Vec<isize>,
}

impl RPN {
    pub fn new() -> Self {
        RPN {
            number: None,
            stack: Vec::new(),
        }
    }

    pub fn handle_byte(&mut self, c: char) -> bool {
        match c {
            '_' | ' ' | '\n' | '0' ..= '9' => self.handle_digit(c),
            '+' | '-' | '*' | '/' | '%' => self.handle_arith(c),
            'd' | 'r' | 'c' => self.handle_stack(c),
            'p' | 'f' => self.handle_print(c),
            'q' => return true,
            _ => {}
        }

        return false
    }

    fn handle_digit(&mut self, c: char) {
        match c {
            '_' => {
                self.number = Some((self.number.map_or(0, |e| e.0), true));
            }
            '0' ..= '9' => {
                let new_dgt = (c as isize) - ('0' as isize);
                self.number = Some((
                    self.number.map_or(new_dgt, |e| if e.1 { e.0 - new_dgt } else { e.0 + new_dgt }),
                    self.number.map_or(false, |e| e.1)
                ));
            }
            ' ' | '\n' => {
                if let Some(e) = &self.number {
                    self.stack.push(e.0);
                    self.number = None;
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
