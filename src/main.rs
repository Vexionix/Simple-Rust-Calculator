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
                while let Some(term) = stack.pop() {
                    if term == Term::LeftParen {
                        break;
                    }
                    postfix.push(term);
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
                let n2 = match new.pop().unwrap() {
                    Term::Number(n) => n,
                    _ => panic!("Syntax error"),
                };
                let n1 = match new.pop().unwrap() {
                    Term::Number(n) => n,
                    _ => panic!("Syntax error"),
                };
                match c {
                    '+' => new.push(Term::Number(n1 + n2)),
                    '-' => new.push(Term::Number(n1 - n2)),
                    '*' => new.push(Term::Number(n1 * n2)),
                    '/' => new.push(Term::Number(n1 / n2)),
                    '^' => new.push(Term::Number(n1.powf(n2))),
                    _ => panic!("Syntax error"),
                }
                for term in terms.iter().skip(i + 1) {
                    new.push(term.clone())
                }
                return new;
            }
            Term::Function(func) => {
                let n = match new.pop().unwrap() {
                    Term::Number(num) => num,
                    _ => panic!("Syntax error"),
                };
                match func.as_str() {
                    "log" => new.push(Term::Number(f64::log10(n))),
                    "sqrt" => new.push(Term::Number(f64::sqrt(n))),
                    "sin" => new.push(Term::Number(f64::sin(n))),
                    "cos" => new.push(Term::Number(f64::cos(n))),
                    "tan" => new.push(Term::Number(f64::tan(n))),
                    "ctg" => new.push(Term::Number(1.0 / f64::tan(n))),
                    _ => panic!("Syntax error"),
                }
                for term in terms.iter().skip(i + 1) {
                    new.push(term.clone())
                }
                return new;
            }
            _ => new.push(terms[i].clone()),
        }
    }

    new
}

fn syntax_check(terms: Vec<Term>) {
    let mut count = 0;
    for (i, term) in terms.clone().into_iter().enumerate() {
        match term {
            Term::LeftParen => {
                count += 1;
                if i + 1 == terms.len() {
                    panic!("Syntax error. Expression shouldn't end this way.");
                } else if !matches!(
                    terms[i + 1],
                    Term::Number(_) | Term::Function(_) | Term::LeftParen
                ) {
                    panic!("Syntax error. Check \'(\'.");
                }
            }
            Term::RightParen => {
                count -= 1;
                if i == 0 {
                    panic!("Syntax error. Expression shouldn't start this way.");
                } else if !matches!(terms[i - 1], Term::Number(_) | Term::RightParen) {
                    panic!("Syntax error. Check \')\' precedence.");
                }
                if i + 1 != terms.len() && !matches!(terms[i + 1], Term::Op(_) | Term::RightParen) {
                    panic!("Syntax error. Check which terms are used after \')\'.");
                }
            }

            Term::Number(_) => {
                if i + 1 < terms.len() && !matches!(terms[i + 1], Term::Op(_) | Term::RightParen) {
                    panic!("Syntax error. Check what comes after numbers.");
                }
            }
            Term::Function(_) => {
                if i + 1 == terms.len() {
                    panic!("Syntax error. Expression shouldn't end this way.");
                } else if !matches!(terms[i + 1], Term::LeftParen) {
                    panic!("Syntax error. Check what comes after functions.");
                }
            }
            Term::Op(_) => {
                if i == 0 || i + 1 == terms.len() {
                    panic!(
                        "Syntax error. Expression starts or ends in a wrong way. Check operands"
                    );
                } else {
                    if !matches!(terms[i - 1], Term::Number(_) | Term::RightParen) {
                        panic!("Syntax error. Check what comes before operators.");
                    }
                    if !matches!(
                        terms[i + 1],
                        Term::Number(_) | Term::Function(_) | Term::LeftParen
                    ) {
                        panic!("Syntax error. Check what comes after operators.");
                    }
                }
            }
        }
    }

    if count != 0 {
        panic!("Syntax error. Parenthesis number is not equal.");
    }
}

fn get_infix_string_from_postfix(postfix_tokens: Vec<Term>) -> String {
    let mut infix: Vec<String> = Vec::new();

    for token in postfix_tokens {
        match token {
            Term::Number(num) => {
                infix.push(num.to_string());
            }
            Term::Function(func) => {
                let operand = infix.pop().unwrap();
                infix.push(format!("{}({})", func, operand));
            }
            Term::Op(operator) => {
                let operand2 = infix.pop().unwrap();
                let operand1 = infix.pop().unwrap();

                infix.push(format!("({} {} {})", operand1, operator, operand2));
            }
            _ => {
                panic!("Invalid postfix expression");
            }
        }
    }

    infix.pop().unwrap()
}

fn main() {
    let input = "(1 + 2 * sin(2 + sqrt(log(3 / 2))))";
    syntax_check(lex(input).clone());
    let mut postfix = infix_to_postfix(lex(input));
    println!("{} =", input);
    while postfix.len() > 1 {
        println!("= {}", get_infix_string_from_postfix(postfix.clone()));
        postfix = eval_first(postfix);
    }
    println!("= {}", get_infix_string_from_postfix(postfix.clone()));
}
