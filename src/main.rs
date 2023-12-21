#[derive(Debug, Clone, PartialEq)]
enum Term {
    Number(f64),
    Op(char),
    Function(String),
    LeftParen,
    RightParen,
}

fn lex(expr: &str) -> Vec<Term> {
    let mut chars = expr.chars().peekable();
    let mut number_buffer = String::new();
    let mut function_buffer = String::new();
    let mut terms = Vec::new();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_ascii_digit() || c == '.' {
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() || c == '.' {
                    number_buffer.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            let number: f64 = match number_buffer.parse::<f64>() {
                Ok(float_value) => float_value,
                Err(_) => panic!("Failed lexing. NAN: {}.", number_buffer),
            };
            terms.push(Term::Number(number));
            number_buffer.clear();
        } else if c.is_ascii_alphabetic() {
            while let Some(&c) = chars.peek() {
                if c.is_ascii_alphabetic() {
                    function_buffer.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            terms.push(match function_buffer.as_str() {
                "log" => Term::Function("log".to_string()),
                "sin" => Term::Function("sin".to_string()),
                "cos" => Term::Function("cos".to_string()),
                "tan" => Term::Function("tan".to_string()),
                "ctg" => Term::Function("ctg".to_string()),
                "sqrt" => Term::Function("sqrt".to_string()),
                _ => panic!(
                    "Failed lexing. Provided inexistent function: {}",
                    function_buffer
                ),
            });
            function_buffer.clear();
        } else {
            terms.push(match c {
                '+' | '-' | '*' | '/' | '^' => Term::Op(c),
                '(' => Term::LeftParen,
                ')' => Term::RightParen,
                _ => panic!(
                    "Failed lexing. Provided inexistent operator or parenthesis: {}",
                    c
                ),
            });
            chars.next();
        }
    }
    terms
}

fn main() {
    let input = "1 + 2 * sin(2 + sqrt(log(3 / 2))) + 3 * 2";
    let terms = lex(input);
    for i in terms {
        match i {
            Term::Number(float_value) => println!("{}", float_value),
            Term::Op(c) => println!("{}", c),
            Term::Sin => println!("sin"),
            Term::Cos => println!("cos"),
            Term::Tan => println!("tan"),
            Term::Ctg => println!("ctg"),
            Term::Sqrt => println!("sqrt"),
            Term::Log => println!("log"),
            Term::LeftParen => println!("("),
            Term::RightParen => println!(")"),
        }
    }
}
