//! An application that emulates a calculator

/// It prints the value of the expression after every step.

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Op(char),
    Function(String),
    LeftParen,
    RightParen,
}

// function for lexing the initial expression (works for valid expressions)
fn lex(expr: &str) -> Vec<Token> {
    let mut chars = expr.chars().peekable();
    let mut number_buffer = String::new();
    let mut function_buffer = String::new();
    let mut tokens = Vec::new();

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
            tokens.push(Token::Number(number));
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
            tokens.push(match function_buffer.as_str() {
                "log" => Token::Function("log".to_string()),
                "sin" => Token::Function("sin".to_string()),
                "cos" => Token::Function("cos".to_string()),
                "tan" => Token::Function("tan".to_string()),
                "ctg" => Token::Function("ctg".to_string()),
                "sqrt" => Token::Function("sqrt".to_string()),
                _ => panic!(
                    "Failed lexing. Provided inexistent function: {}",
                    function_buffer
                ),
            });
            function_buffer.clear();
        } else {
            tokens.push(match c {
                '+' | '-' | '*' | '/' | '^' => Token::Op(c),
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                _ => panic!(
                    "Failed lexing. Provided inexistent operator or parenthesis: {}",
                    c
                ),
            });
            chars.next();
        }
    }
    tokens
}

// returns the postfix format of the items found in tokens
fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut postfix = Vec::new();
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => postfix.push(token),
            Token::Op(_) => {
                while !stack.is_empty() && (priority(stack.last().unwrap()) >= priority(&token)) {
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(token);
            }
            Token::Function(_) => stack.push(token),
            Token::LeftParen => stack.push(token),
            Token::RightParen => {
                while let Some(token) = stack.pop() {
                    if token == Token::LeftParen {
                        break;
                    }
                    postfix.push(token);
                }
                if let Some(Token::Function(_)) = stack.last() {
                    postfix.push(stack.pop().unwrap());
                }
            }
        }
    }

    while let Some(token) = stack.pop() {
        postfix.push(token);
    }

    postfix
}

// used for priority order when converting to postfix
fn priority(op: &Token) -> i32 {
    match op {
        Token::Op('+') | Token::Op('-') => 1,
        Token::Op('*') | Token::Op('/') => 2,
        Token::Op('^') => 3,
        _ => 0,
    }
}

// function used to evaluate the current step (only calculates the highest priority operation)
fn eval_first(tokens: Vec<Token>) -> Vec<Token> {
    let mut new: Vec<Token> = Vec::new();
    for i in 0..tokens.len() {
        match tokens[i].clone() {
            Token::Op(c) => {
                let n2 = match new.pop().unwrap() {
                    Token::Number(n) => n,
                    _ => panic!("Syntax error"),
                };
                let n1 = match new.pop().unwrap() {
                    Token::Number(n) => n,
                    _ => panic!("Syntax error"),
                };
                match c {
                    '+' => new.push(Token::Number(n1 + n2)),
                    '-' => new.push(Token::Number(n1 - n2)),
                    '*' => new.push(Token::Number(n1 * n2)),
                    '/' => new.push(Token::Number(n1 / n2)),
                    '^' => new.push(Token::Number(n1.powf(n2))),
                    _ => panic!("Syntax error"),
                }
                for token in tokens.iter().skip(i + 1) {
                    new.push(token.clone())
                }
                return new;
            }
            Token::Function(func) => {
                let n = match new.pop().unwrap() {
                    Token::Number(num) => num,
                    _ => panic!("Syntax error"),
                };
                match func.as_str() {
                    "log" => new.push(Token::Number(f64::log10(n))),
                    "sqrt" => new.push(Token::Number(f64::sqrt(n))),
                    "sin" => new.push(Token::Number(f64::sin(n))),
                    "cos" => new.push(Token::Number(f64::cos(n))),
                    "tan" => new.push(Token::Number(f64::tan(n))),
                    "ctg" => new.push(Token::Number(1.0 / f64::tan(n))),
                    _ => panic!("Syntax error"),
                }
                for token in tokens.iter().skip(i + 1) {
                    new.push(token.clone())
                }
                return new;
            }
            _ => new.push(tokens[i].clone()),
        }
    }

    new
}

// function used for syntax checking. If the expression is invalid the program will stop running
fn syntax_check(tokens: Vec<Token>) {
    let mut count = 0;
    for (i, token) in tokens.clone().into_iter().enumerate() {
        match token {
            Token::LeftParen => {
                count += 1;
                if i + 1 == tokens.len() {
                    panic!("Syntax error. Expression shouldn't end this way.");
                } else if !matches!(
                    tokens[i + 1],
                    Token::Number(_) | Token::Function(_) | Token::LeftParen
                ) {
                    panic!("Syntax error. Check \'(\'.");
                }
            }
            Token::RightParen => {
                count -= 1;
                if i == 0 {
                    panic!("Syntax error. Expression shouldn't start this way.");
                } else if !matches!(tokens[i - 1], Token::Number(_) | Token::RightParen) {
                    panic!("Syntax error. Check \')\' precedence.");
                }
                if i + 1 != tokens.len()
                    && !matches!(tokens[i + 1], Token::Op(_) | Token::RightParen)
                {
                    panic!("Syntax error. Check which tokens are used after \')\'.");
                }
            }

            Token::Number(_) => {
                if i + 1 < tokens.len()
                    && !matches!(tokens[i + 1], Token::Op(_) | Token::RightParen)
                {
                    panic!("Syntax error. Check what comes after numbers.");
                }
            }
            Token::Function(_) => {
                if i + 1 == tokens.len() {
                    panic!("Syntax error. Expression shouldn't end this way.");
                } else if !matches!(tokens[i + 1], Token::LeftParen) {
                    panic!("Syntax error. Check what comes after functions.");
                }
            }
            Token::Op(_) => {
                if i == 0 || i + 1 == tokens.len() {
                    panic!(
                        "Syntax error. Expression starts or ends in a wrong way. Check operands"
                    );
                } else {
                    if !matches!(tokens[i - 1], Token::Number(_) | Token::RightParen) {
                        panic!("Syntax error. Check what comes before operators.");
                    }
                    if !matches!(
                        tokens[i + 1],
                        Token::Number(_) | Token::Function(_) | Token::LeftParen
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

// converts the postfix tokens into a string that contains the infix format of the expression
fn get_infix_string_from_postfix(postfix_tokens: Vec<Token>) -> String {
    let mut infix: Vec<String> = Vec::new();

    for token in postfix_tokens {
        match token {
            Token::Number(num) => {
                infix.push(num.to_string());
            }
            Token::Function(func) => {
                let operand = infix.pop().unwrap();
                infix.push(format!("{}({})", func, operand));
            }
            Token::Op(operator) => {
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
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    syntax_check(lex(input.trim()).clone());
    let mut postfix = infix_to_postfix(lex(input.trim()));
    println!("{} =", input.trim());
    while postfix.len() > 1 {
        println!("= {}", get_infix_string_from_postfix(postfix.clone()));
        postfix = eval_first(postfix);
    }
    println!("= {}", get_infix_string_from_postfix(postfix.clone()));
}
