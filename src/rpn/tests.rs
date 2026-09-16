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

    assert_eq!(rpn.stack.last(), Some(&((99 + 81) as f32, -1 as isize)));
}

#[test]
fn addition_negative() {
    let mut rpn = RPN::new();
    write_bytes(&mut rpn, "99 _81 +\n");

    assert_eq!(rpn.stack.last(), Some(&((99 + -81) as f32, -1 as isize)));
}

#[test]
fn substraction() {
    let mut rpn = RPN::new();
    write_bytes(&mut rpn, "99 81 -\n");

    assert_eq!(rpn.stack.last(), Some(&((99.0 - 81.0) as f32, -1 as isize)));
}

#[test]
fn substraction_negative() {
    let mut rpn = RPN::new();
    write_bytes(&mut rpn, "81 99 -\n");

    assert_eq!(rpn.stack.last(), Some(&((81.0 - 99.0) as f32, -1 as isize)));
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
