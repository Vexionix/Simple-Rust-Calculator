fn eval_expression(expr: &str) -> bool {
    let mut chars = expr.chars().peekable();
    let mut number_buffer = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if (c >= '0' && c <= '9') || c == '.' {
            while let Some(&c) = chars.peek() {
                if (c >= '0' && c <= '9') || c == '.' {
                    number_buffer.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            let number: f64;
            match number_buffer.parse::<f64>() {
                Ok(float_value) => {
                    number = float_value;
                }
                Err(_) => return false,
            }
            number_buffer.clear();
            println!("{}", number);
        } else {
            match c {
                '+' | '-' | '*' | '/' | '^' | '(' | ')' => {}
                _ => return false,
            };
            chars.next();
        }
    }
    return true;
}

fn main() {
    let input = "1.53 + 3 - 2";
    if eval_expression(input) == true {
        println!("Correct.");
    } else {
        println!("Incorrect.");
    }
}
