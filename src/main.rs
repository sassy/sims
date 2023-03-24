use std::{collections::HashMap, io::Write};

mod tokenize;

#[derive(Debug,Clone, PartialEq)]
enum LispExpr {
    Int(i32),
    Symbol(String),
    List(Vec<LispExpr>),
    Cons(Box<(LispExpr, LispExpr)>),
}

impl LispExpr {
    pub fn expr_str(self) -> String {
        match self {
            LispExpr::Int(n) => n.to_string(),
            LispExpr::Symbol(s) => s,
            LispExpr::List(_) => "not implemented".to_string(),
            LispExpr::Cons(_) => "not implemented".to_string(),
        }
    }
}

// tokenを構文木に変換
fn parse(tokens: &[String]) -> Result<(LispExpr, &[String]), String> {
    if tokens.is_empty() {
        return Err("Unexpected end of input".to_string());
    }

    let token = tokens[0].clone();
    let mut rest= &tokens[1..];

    if token == "(" {
        let mut list = Vec::new();
        loop {
            let (expr, new_rest) = parse(rest)?;
            rest = new_rest;
            if rest.is_empty() {
                return  Err("Unexpected end of input".to_string());
            }
            list.push(expr);
            if rest[0] == ")" {
                rest = &rest[1..];
                if list.len() == 3 && list[0] == LispExpr::Symbol("cons".to_string()) {
                    let car = list[1].clone();
                    let cdr = list[2].clone();
                    let cons = LispExpr::Cons(Box::new((car, cdr)));
                    return Ok((cons, rest));
                }
                return Ok((LispExpr::List(list), rest));
            }
        }        
    } else if token == ")" {
        return Err("Unexpected ')".to_string());
    } else {
        if let Ok(int) = token.parse::<i32>() {
            return Ok((LispExpr::Int(int), rest));
        } else {
            return Ok((LispExpr::Symbol(token), rest));
        }
    }
}

// 構文木を再帰的に評価する
fn eval(expr: &LispExpr, env: &mut HashMap<String, i32>) -> Result<LispExpr, String> {
    match expr {
        LispExpr::Int(n) => Ok(LispExpr::Int(*n)),
        LispExpr::Symbol(s) => {
            if let Some(val) = env.get(s) {
                Ok(LispExpr::Int(*val))
            } else {
                Err(format!("Undefined symbol '{}", s))
            }
        }
        LispExpr::List(list) => {
            if list.is_empty() {
                return Err("Empty list".to_string());
            }

            let first = &list[0];
            let rest = &list[1..];
            match first {
                LispExpr::Symbol(s) if s == "+" => {
                    let mut result = 0;
                    for expr in rest {
                        let tmp = eval(expr, env)?;
                        match tmp {
                            LispExpr::Int(n) => result += n,
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(LispExpr::Int(result))
                }
                LispExpr::Symbol(s) if s == "-" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let first = eval(&rest[0], env)?;
                    let mut result;
                    match first {
                        LispExpr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let tmp  = eval(expr, env)?;
                        match tmp {
                            LispExpr::Int(n) => result -= n,
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(LispExpr::Int(result))
                }
                LispExpr::Symbol(s) if s == "*" => {
                    let mut result = 1;
                    for expr in rest {
                        let tmp = eval(expr, env)?;
                        match tmp {
                            LispExpr::Int(n) => result *=  n,
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(LispExpr::Int(result))
                }
                LispExpr::Symbol(s) if s == "/" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let first = eval(&rest[0], env)?;
                    let mut result;
                    match first {
                        LispExpr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let tmp = eval(expr, env)?;
                        let val;
                        match tmp {
                            LispExpr::Int(n) => val = n,
                            _ => panic!("Unexpected argument")
                        }
                        if val == 0 {
                            return Err("Divison by zero".to_string());
                        }
                        result /= val;
                    }
                    Ok(LispExpr::Int(result))                    
                }
                LispExpr::Symbol(s) if s == "=" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let result = eval(&rest[0], env)?;
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        if val != result {
                            return Ok(LispExpr::Int(0));
                        }
                    }
                    Ok(LispExpr::Int(1))                    
                }
                LispExpr::Symbol(s) if s == "<" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let result;
                    match tmp {
                        LispExpr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        match val {
                            LispExpr::Int(n) => {
                                if n <= result {
                                    return Ok(LispExpr::Int(0));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(LispExpr::Int(1))                    
                }
                LispExpr::Symbol(s) if s == "<=" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let result;
                    match tmp {
                        LispExpr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        match val {
                            LispExpr::Int(n) => {
                                if n < result {
                                    return Ok(LispExpr::Int(0));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(LispExpr::Int(1))                    
                }
                LispExpr::Symbol(s) if s == ">" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let result;
                    match tmp {
                        LispExpr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        match val {
                            LispExpr::Int(n) => {
                                if n >= result {
                                    return Ok(LispExpr::Int(0));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(LispExpr::Int(1))                    
                }
                LispExpr::Symbol(s) if s == ">=" => {
                    if rest.is_empty() {
                        return Err("Expected at least one argument".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let result;
                    match tmp {
                        LispExpr::Int(n) => result = n,
                        _ => panic!("Unexpected argument")
                    }
                    for expr in &rest[1..] {
                        let val = eval(expr, env)?;
                        match val {
                            LispExpr::Int(n) => {
                                if n > result {
                                    return Ok(LispExpr::Int(0));
                                }
                            },
                            _ => panic!("Unexpected argument")
                        }
                    }
                    Ok(LispExpr::Int(1))                    
                }
                LispExpr::Symbol(s) if s == "let" => {
                    if rest.len() != 2 {
                        return Err("Expected two arguments".to_string());
                    }
                    if let LispExpr::Symbol(name) = &rest[0] {
                        let val = eval(&rest[1], env)?;
                        match val {
                            LispExpr::Int(n) => {
                                env.insert(name.clone(), n);
                                return Ok(val);
                            },
                            _ => panic!("Unexpected argument")
                        }                        
                    } else {
                        Err("Expected a symbol as first argument".to_string())
                    }
                }
                LispExpr::Symbol(s) if s == "if" => {
                    if rest.len() != 3 {
                        return Err("Expected three arguments".to_string());
                    }
                    let tmp = eval(&rest[0], env)?;
                    let pred;
                    match tmp {
                        LispExpr::Int(n) => pred = n,
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
        LispExpr::Cons(_) => unimplemented!(),
    }
}

fn eval_str(program: &str) -> Result<LispExpr, String> {
    let tokens = tokenize::tokenize(program);
    let (expr, rest) = parse(&tokens)?;
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
    use crate::{parse, eval_str, LispExpr};

    #[test]
    fn test_parse() {
        let tokens = vec![
            "(".to_string(),
            "+".to_string(),
            "2".to_string(),
            "3".to_string(),
            ")".to_string()
        ];
        let (expr, rest) = parse(&tokens).unwrap();
        assert_eq!(expr, LispExpr::List(vec![
            LispExpr::Symbol("+".to_string()),
            LispExpr::Int(2),
            LispExpr::Int(3),
        ]));
        assert_eq!(rest, Vec::<String>::new());
    }
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