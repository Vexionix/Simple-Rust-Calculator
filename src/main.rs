fn eval_expression(expr: &str) -> bool {
    let expr_copy = expr.clone();
    for part in expr_copy.split(" ") {
        match part {
            "+" => {}
            "-" => {}
            "*" => {}
            "/" => {}
            _ => {
                match part.parse::<f64>() {
                    Ok(_float_value) => { // temporary
                    }
                    Err(_) => return false,
                }
            }
        }
    }
    return true;
}

fn main() {
    let input = "1.5 + 3 - 2";
    if eval_expression(input) == true {
        println!("Correct.");
    } else {
        println!("Incorrect.");
    }
}
