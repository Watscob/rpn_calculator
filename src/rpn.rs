use std::io::{self, Read};

pub struct RPN {
    buffer: String,
    stack: Vec<(f32, isize)>,
    precision: usize,
}

impl RPN {
    pub fn new() -> Self {
        RPN {
            buffer: String::new(),
            stack: Vec::new(),
            precision: 0,
        }
    }

    pub fn run(&mut self) {
        for byte in io::stdin().lock().bytes() {
            let quit = match byte {
                Ok(b) => self.handle_byte(b as char),
                Err(_) => false,
            };

            if quit {
                return;
            }
        }
    }

    fn handle_byte(&mut self, c: char) -> bool {
        let _ = self.handle_digit(c)
             || self.handle_arith(c)
             || self.handle_stack(c)
             || self.handle_print(c)
             || self.handle_precision(c);

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
                        self.stack.push((num, -1));
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
                    self.stack.push((op1.0 + op2.0, self.precision as isize));
                }
                true
            }
            '-' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push((op1.0 - op2.0, self.precision as isize));
                }
                true
            }
            '*' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push((op1.0 * op2.0, self.precision as isize));
                }
                true
            }
            '/' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push((op1.0 / op2.0, self.precision as isize));
                }
                true
            }
            '%' => {
                if let (Some(op2), Some(op1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push((op1.0 % op2.0, self.precision as isize));
                }
                true
            }
            _ => false
        }
    }

    fn handle_print(&mut self, c: char) -> bool {
        match c {
            'p' => {
                println!("{}", self.stack.last().map_or("Stack is empty".to_string(), |&(n, p)| if p == -1 { n.to_string() } else { format!("{:.1$}", n, p as usize) }));
                true
            }
            'n' => {
                print!("{}", self.stack.pop().map_or("Stack is empty".to_string(), |(n, p)| if p == -1 { n.to_string() } else { format!("{:.1$}", n, p as usize) }));
                true
            }
            'f' => {
                if self.stack.is_empty() {
                    println!("Stack is empty");
                } else {
                    for &(n, p) in self.stack.iter().rev() {
                        println!("{}", if p == -1 { n.to_string() } else { format!("{:.1$}", n, p as usize) });
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

    fn handle_precision(&mut self, c: char) -> bool {
        match c {
            'k' => {
                if let Some((n, _)) = self.stack.pop() {
                    self.precision = n as usize;
                }
                true
            }
            'K' => {
                self.stack.push((self.precision as f32, -1));
                true
            }
            _ => false
        }
    }
}

#[cfg(test)]
mod test {
    use super::RPN;

    fn write_bytes(rpn: &mut RPN, s: &str) {
        for c in s.chars() {
            rpn.handle_byte(c);
        }
    }

    #[test]
    fn parse_integer() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "981 ");

        assert_eq!(rpn.stack.last(), Some(&(981 as f32, -1 as isize)));
    }

    #[test]
    fn parse_float() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "98.1 ");

        assert_eq!(rpn.stack.last(), Some(&(98.1 as f32, -1 as isize)));
    }

    #[test]
    fn parse_negative() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "_981\n");

        assert_eq!(rpn.stack.last(), Some(&(-981 as f32, -1 as isize)));
    }

    #[test]
    fn addition() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "99 81 +\n");

        assert_eq!(rpn.stack.last(), Some(&((99 + 81) as f32, 0 as isize)));
    }

    #[test]
    fn addition_negative() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "99 _81 +\n");

        assert_eq!(rpn.stack.last(), Some(&((99 + -81) as f32, 0 as isize)));
    }

    #[test]
    fn substraction() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "99 81 -\n");

        assert_eq!(rpn.stack.last(), Some(&((99.0 - 81.0) as f32, 0 as isize)));
    }

    #[test]
    fn substraction_negative() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "81 99 -\n");

        assert_eq!(rpn.stack.last(), Some(&((81.0 - 99.0) as f32, 0 as isize)));
    }

    #[test]
    fn multiplication() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "99 81 *\n");

        assert_eq!(rpn.stack.last(), Some(&((99.0 * 81.0) as f32, 0 as isize)));
    }

    #[test]
    fn multiplication_negative() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "_81 99 *\n");

        assert_eq!(rpn.stack.last(), Some(&((-81.0 * 99.0) as f32, 0 as isize)));
    }

    #[test]
    fn division() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "99 81 /\n");

        assert_eq!(rpn.stack.last(), Some(&((99.0 / 81.0) as f32, 0 as isize)));
    }

    #[test]
    fn division_negative() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "_81 99 /\n");

        assert_eq!(rpn.stack.last(), Some(&((-81.0 / 99.0) as f32, 0 as isize)));
    }

    #[test]
    fn modulo() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "99 81 %\n");

        assert_eq!(rpn.stack.last(), Some(&((99.0 % 81.0) as f32, 0 as isize)));
    }

    #[test]
    fn modulo_negative() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "_81 99 % ");

        assert_eq!(rpn.stack.last(), Some(&((-81.0 % 99.0) as f32, 0 as isize)));
    }

    #[test]
    fn clear_stack() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "99 98 97 96 95 ");
        assert_ne!(rpn.stack.len(), 0);
        write_bytes(&mut rpn, "c ");
        assert_eq!(rpn.stack.len(), 0);
    }

    #[test]
    fn duplicate_last_element() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "99 98 97 96 95 ");
        assert_eq!(rpn.stack.len(), 5);
        assert_eq!(rpn.stack.get(rpn.stack.len() - 2), Some(&(96 as f32, -1 as isize)));
        assert_eq!(rpn.stack.get(rpn.stack.len() - 1), Some(&(95 as f32, -1 as isize)));
        write_bytes(&mut rpn, "d ");
        assert_eq!(rpn.stack.len(), 6);
        assert_eq!(rpn.stack.get(rpn.stack.len() - 2), Some(&(95 as f32, -1 as isize)));
        assert_eq!(rpn.stack.get(rpn.stack.len() - 1), Some(&(95 as f32, -1 as isize)));
    }

    #[test]
    fn reverse_last_elements() {
        let mut rpn = RPN::new();
        write_bytes(&mut rpn, "99 98 97 96 95 ");
        assert_eq!(rpn.stack.len(), 5);
        assert_eq!(rpn.stack.get(rpn.stack.len() - 2), Some(&(96 as f32, -1 as isize)));
        assert_eq!(rpn.stack.get(rpn.stack.len() - 1), Some(&(95 as f32, -1 as isize)));
        write_bytes(&mut rpn, "r ");
        assert_eq!(rpn.stack.len(), 5);
        assert_eq!(rpn.stack.get(rpn.stack.len() - 2), Some(&(95 as f32, -1 as isize)));
        assert_eq!(rpn.stack.get(rpn.stack.len() - 1), Some(&(96 as f32, -1 as isize)));
    }

    #[test]
    fn set_precision() {
        let mut rpn = RPN::new();
        assert_eq!(rpn.precision, 0);
        write_bytes(&mut rpn, "4 k ");
        assert_eq!(rpn.precision, 4);
    }

    #[test]
    fn push_precision_on_stack() {
        let mut rpn = RPN::new();
        assert_eq!(rpn.precision, 0);
        write_bytes(&mut rpn, "4 k ");
        assert_eq!(rpn.precision, 4);
        write_bytes(&mut rpn, "K ");
        assert_eq!(rpn.stack.last(), Some(&(4 as f32, -1 as isize)));
    }
}
