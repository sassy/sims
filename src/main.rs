use std::{collections::HashMap, io::Write};

mod tokenize;
mod expr;
mod parse;

// 構文木を再帰的に評価する
fn eval(expr: &expr::Expr, env: &mut HashMap<String, i32>) -> Result<expr::Expr, String> {
    match expr {
        expr::Expr::Int(n) => Ok(expr::Expr::Int(*n)),
        expr::Expr::Symbol(s) => {
            if let Some(val) = env.get(s) {
                Ok(expr::Expr::Int(*val))
            } else {
                Err(format!("Undefined symbol '{}", s))
            }
        }
        expr::Expr::List(list) => {
            if list.is_empty() {
                return Err("Empty list".to_string());
            }

            let first = &list[0];
            let rest = &list[1..];
            match first {
                expr::Expr::Symbol(s) if s == "+" => {
                    let mut result = 0;
                    for expr in rest {
                        let tmp = eval(expr, env)?;
                        match tmp {
                            expr::Expr::Int(n) => result += n,
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Int(result))
                }
                expr::Expr::Symbol(s) if s == "-" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let first = eval(&rest[0], env)?;
                    let mut result;
                    match first {
                        expr::Expr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let tmp  = eval(expr, env)?;
                        match tmp {
                            expr::Expr::Int(n) => result -= n,
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Int(result))
                }
                expr::Expr::Symbol(s) if s == "*" => {
                    let mut result = 1;
                    for expr in rest {
                        let tmp = eval(expr, env)?;
                        match tmp {
                            expr::Expr::Int(n) => result *=  n,
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Int(result))
                }
                expr::Expr::Symbol(s) if s == "/" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let first = eval(&rest[0], env)?;
                    let mut result;
                    match first {
                        expr::Expr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let tmp = eval(expr, env)?;
                        let val;
                        match tmp {
                            expr::Expr::Int(n) => val = n,
                            _ => panic!("Unexpected argument")
                        }
                        if val == 0 {
                            return Err("Divison by zero".to_string());
                        }
                        result /= val;
                    }
                    Ok(expr::Expr::Int(result))                    
                }
                expr::Expr::Symbol(s) if s == "=" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let result = eval(&rest[0], env)?;
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        if val != result {
                            return Ok(expr::Expr::Int(0));
                        }
                    }
                    Ok(expr::Expr::Int(1))                    
                }
                expr::Expr::Symbol(s) if s == "<" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let result;
                    match tmp {
                        expr::Expr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        match val {
                            expr::Expr::Int(n) => {
                                if n <= result {
                                    return Ok(expr::Expr::Int(0));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Int(1))                    
                }
                expr::Expr::Symbol(s) if s == "<=" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let result;
                    match tmp {
                        expr::Expr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        match val {
                            expr::Expr::Int(n) => {
                                if n < result {
                                    return Ok(expr::Expr::Int(0));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Int(1))                    
                }
                expr::Expr::Symbol(s) if s == ">" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let result;
                    match tmp {
                        expr::Expr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        match val {
                            expr::Expr::Int(n) => {
                                if n >= result {
                                    return Ok(expr::Expr::Int(0));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Int(1))                    
                }
                expr::Expr::Symbol(s) if s == ">=" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let result;
                    match tmp {
                        expr::Expr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        match val {
                            expr::Expr::Int(n) => {
                                if n > result {
                                    return Ok(expr::Expr::Int(0));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Int(1))                    
                }
                expr::Expr::Symbol(s) if s == "let" => {
                    if rest.len() != 2 {
                        return Err("Expected two arguments".to_string());
                    }
                    if let expr::Expr::Symbol(name) = &rest[0] {
                        let val = eval(&rest[1], env)?;
                        match val {
                            expr::Expr::Int(n) => {
                                env.insert(name.clone(), n);
                                return Ok(val);
                            },
                            _ => panic!("Unexpected argument")
                        }                        
                    } else {
                        Err("Expected a symbol as first argument".to_string())
                    }
                }
                expr::Expr::Symbol(s) if s == "if" => {
                    if rest.len() != 3 {
                        return Err("Expected three arguments".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let pred;
                    match tmp {
                        expr::Expr::Int(n) => pred = n,
                        _ => panic!("Unexpected argument")
                    }
                    if pred != 0 {
                        eval(&rest[1], env)
                    } else {
                        eval(&rest[2], env)
                    }
                }
                _ => Err("Unexpected function or syntax".to_string()),
            }
        }
        expr::Expr::Cons(_) => unimplemented!(),
    }
}

fn eval_str(program: &str) -> Result<expr::Expr, String> {
    let tokens = tokenize::tokenize(program);
    let (expr, rest) = parse::parse(&tokens)?;
    if !rest.is_empty() {
        return Err("Unexpected trailing tokens".to_string());
    }
    let mut env = HashMap::new();
    eval(&expr, &mut env)
}


fn main() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "" {
            continue;
        }
        if input == "quit" {
            break;
        }
        match eval_str(input) {
            Ok(result) => println!("{}", result.expr_str()),
            Err(err) => println!("Error: {}", err)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_str;

   
    #[test]
    fn test_eval_str() {
        let result = eval_str("(* 2 3)").unwrap().expr_str();
        assert_eq!(result, "6".to_string());
        let result = eval_str("(- 7 4)").unwrap().expr_str();
        assert_eq!(result, "3".to_string());
        let result = eval_str("(if 1 3 2)").unwrap().expr_str();
        assert_eq!(result, "3".to_string());
        let result = eval_str("(= 1 3 2)").unwrap().expr_str();
        assert_eq!(result, "0".to_string());
        let result = eval_str("(= 1 1 1)").unwrap().expr_str();
        assert_eq!(result, "1".to_string());
        let result = eval_str("(< 1 2)").unwrap().expr_str();
        assert_eq!(result, "1".to_string());
        let result = eval_str("(<= 1 1)").unwrap().expr_str();
        assert_eq!(result, "1".to_string());
        let result = eval_str("(> 2 1)").unwrap().expr_str();
        assert_eq!(result, "1".to_string());
        let result = eval_str("(>= 1 1)").unwrap().expr_str();
        assert_eq!(result, "1".to_string());
    }
}