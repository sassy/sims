use crate::environment::Env;
use crate::expr;
use crate::tokenize;
use crate::parse;

// 構文木を再帰的に評価する
fn eval(expr: &expr::Expr, env: &mut Env) -> Result<expr::Expr, String> {
    match expr {
        expr::Expr::Int(n) => Ok(expr::Expr::Int(*n)),
        expr::Expr::Symbol(s) => {
            let val = env.get(s);
            match val {
                expr::Expr::Int(n) => Ok(expr::Expr::Int(n)),
                _ =>  Err(format!("Undefined symbol '{}", s))
            }
        }
        expr::Expr::Boolean(b) => Ok(expr::Expr::Boolean(*b)),
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
                            return Ok(expr::Expr::Boolean(false));
                        }
                    }
                    Ok(expr::Expr::Boolean(true))                    
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
                                    return Ok(expr::Expr::Boolean(false));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Boolean(true))                    
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
                                    return Ok(expr::Expr::Boolean(true));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Boolean(true))                    
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
                                    return Ok(expr::Expr::Boolean(false));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Boolean(true))                    
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
                                    return Ok(expr::Expr::Boolean(false));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(expr::Expr::Boolean(true))                    
                }
                expr::Expr::Symbol(s) if s == "car" => {
                    let arg = &eval(&list[1], env)?;
                    if let expr::Expr::List(items) = arg {
                        if items.len() > 0 {
                            Ok(items[0].clone())
                        } else {
                            Err("Empty list".to_string())
                        }
                    } else {
                        Err("not a list".to_string())
                    }
                }
                expr::Expr::Symbol(s) if s == "cdr" => {
                    let arg = &eval(&list[1], env)?;
                    if let expr::Expr::List(items) = arg {
                        if items.len() > 1 {
                            Ok(expr::Expr::List(items[1..].to_vec()))
                        } else {
                            Err("Empty list".to_string())
                        }
                    } else {
                        Err("not a list".to_string())
                    }
                }
                expr::Expr::Symbol(s) if s == "cons" => {
                    let first = &eval(&list[1], env)?;
                    let rest = &eval(&list[2], env)?;
                    if let expr::Expr::List(items) = rest {
                        let mut new_list = vec![first.clone()];
                        new_list.extend_from_slice(&items[..]);
                        Ok(expr::Expr::List(new_list))
                    } else {
                        Err("not a list".to_string())
                    }
                }
                expr::Expr::Symbol(s) if s == "let" => {
                    if rest.len() != 2 {
                        return Err("Expected two arguments".to_string());
                    }
                    if let expr::Expr::Symbol(name) = &rest[0] {
                        let val = eval(&rest[1], env)?;
                        match val {
                            expr::Expr::Int(_) => {
                                env.set(name.clone(), val.clone());
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
                        expr::Expr::Boolean(b) => pred = b,
                        _ => panic!("Unexpected argument")
                    }
                    if pred  {
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

pub fn eval_str(program: &str, env: &mut Env) -> Result<expr::Expr, String> {
    let tokens = tokenize::tokenize(program);
    let (expr, rest) = parse::parse(&tokens)?;
    if !rest.is_empty() {
        return Err("Unexpected trailing tokens".to_string());
    }
    eval(&expr, env)
}

#[cfg(test)]
mod tests {
    use crate::eval::eval_str;
    use crate::environment::Env;
   
    #[test]
    fn test_eval_str() {
        let mut env = Env::new();

        let result = eval_str("(* 2 3)", &mut env).unwrap().expr_str();
        assert_eq!(result, "6".to_string());
        let result = eval_str("(- 7 4)", &mut env).unwrap().expr_str();
        assert_eq!(result, "3".to_string());
        let result = eval_str("(if (= 1 1) 3 2)", &mut env).unwrap().expr_str();
        assert_eq!(result, "3".to_string());
        let result = eval_str("(= 1 3 2)", &mut env).unwrap().expr_str();
        assert_eq!(result, "#f".to_string());
        let result = eval_str("(= 1 1 1)", &mut env).unwrap().expr_str();
        assert_eq!(result, "#t".to_string());
        let result = eval_str("(< 1 2)", &mut env).unwrap().expr_str();
        assert_eq!(result, "#t".to_string());
        let result = eval_str("(<= 1 1)", &mut env).unwrap().expr_str();
        assert_eq!(result, "#t".to_string());
        let result = eval_str("(> 2 1)", &mut env).unwrap().expr_str();
        assert_eq!(result, "#t".to_string());
        let result = eval_str("(>= 1 1)", &mut env).unwrap().expr_str();
        assert_eq!(result, "#t".to_string());
        let result = eval_str("(< 1 1)", &mut env).unwrap().expr_str();
        assert_eq!(result, "#f".to_string());
    }
}