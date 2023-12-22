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

fn infix_to_postfix(terms: Vec<Term>) -> Vec<Term> {
    let mut postfix = Vec::new();
    let mut stack = Vec::new();

    for term in terms {
        match term {
            Term::Number(_) => postfix.push(term),
            Term::Op(_) => {
                while !stack.is_empty() && (priority(stack.last().unwrap()) >= priority(&term)) {
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(term);
            }
            Term::Function(_) => stack.push(term),
            Term::LeftParen => stack.push(term),
            Term::RightParen => {
                while let Some(top) = stack.pop() {
                    if top == Term::LeftParen {
                        break;
                    }
                    postfix.push(top);
                }
                if let Some(Term::Function(_)) = stack.last() {
                    postfix.push(stack.pop().unwrap());
                }
            }
        }
    }

    while let Some(term) = stack.pop() {
        postfix.push(term);
    }

    postfix
}

fn priority(op: &Term) -> i32 {
    match op {
        Term::Op('+') | Term::Op('-') => 1,
        Term::Op('*') | Term::Op('/') => 2,
        Term::Op('^') => 3,
        _ => 0,
    }
}

fn eval_first(terms: Vec<Term>) -> Vec<Term> {
    let mut new: Vec<Term> = Vec::new();
    for i in 0..terms.len() {
        match terms[i].clone() {
            Term::Op(c) => {
                let n1: f64;
                let n2: f64;
                match new.pop().unwrap() {
                    Term::Number(n) => n2 = n,
                    _ => panic!("Syntax error"),
                }
                match new.pop().unwrap() {
                    Term::Number(n) => n1 = n,
                    _ => panic!("Syntax error"),
                }
                match c {
                    '+' => new.push(Term::Number(n1 + n2)),
                    '-' => new.push(Term::Number(n1 - n2)),
                    '*' => new.push(Term::Number(n1 * n2)),
                    '/' => new.push(Term::Number(n1 / n2)),
                    '^' => new.push(Term::Number(n1.powf(n2))),
                    _ => panic!("Syntax error"),
                }
                for j in i + 1..terms.len() {
                    new.push(terms[j].clone())
                }
                return new;
            }
            Term::Function(func) => {
                let n: f64;
                match new.pop().unwrap() {
                    Term::Number(num) => n = num,
                    _ => panic!("Syntax error"),
                }
                match func.as_str() {
                    "log" => new.push(Term::Number(f64::log10(n))),
                    "sqrt" => new.push(Term::Number(f64::sqrt(n))),
                    "sin" => new.push(Term::Number(f64::sin(n))),
                    "cos" => new.push(Term::Number(f64::cos(n))),
                    "tan" => new.push(Term::Number(f64::tan(n))),
                    "ctg" => new.push(Term::Number(1.0 / f64::tan(n))),
                    _ => panic!("Syntax error"),
                }
                for j in i + 1..terms.len() {
                    new.push(terms[j].clone())
                }
                return new;
            }
            _ => new.push(terms[i].clone()),
        }
    }

    new
}

fn main() {
    let input = "1 + 2 * sin((((2 + sqrt(log(3 / 2)))))) + 3 * 2";
    let terms = lex(input);
    let mut postfix = infix_to_postfix(terms.clone());
    println!("before:   {:?}", terms);
    println!("after: {:?}", postfix);
    while postfix.len() > 1 {
        postfix = eval_first(postfix);
        print!("\n");
        println!("update: {:?}", postfix);
    }
}
