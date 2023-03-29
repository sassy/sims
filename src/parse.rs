use crate::expr;

// tokenを構文木に変換
pub fn parse(tokens: &[String]) -> Result<(expr::Expr, &[String]), String> {
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
                if list.len() == 3 && list[0] == expr::Expr::Symbol("cons".to_string()) {
                    let car = list[1].clone();
                    let cdr = list[2].clone();
                    let cons = expr::Expr::Cons(Box::new((car, cdr)));
                    return Ok((cons, rest));
                }
                return Ok((expr::Expr::List(list), rest));
            }
        }        
    } else if token == ")" {
        return Err("Unexpected ')".to_string());
    } else {
         if token == "#t" {
            Ok((expr::Expr::Boolean(true), rest))
        } else if token == "#f" {
            Ok((expr::Expr::Boolean(false), rest))
        } else if let Ok(int) = token.parse::<i32>() {
            return Ok((expr::Expr::Int(int), rest));
        } else {
            return Ok((expr::Expr::Symbol(token), rest));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse, expr};

    #[test]
    fn test_parse() {
        let tokens = vec![
            "(".to_string(),
            "+".to_string(),
            "2".to_string(),
            "3".to_string(),
            ")".to_string()
        ];
        let (expr, rest) = parse::parse(&tokens).unwrap();
        assert_eq!(expr, expr::Expr::List(vec![
            expr::Expr::Symbol("+".to_string()),
            expr::Expr::Int(2),
            expr::Expr::Int(3),
        ]));
        assert_eq!(rest, Vec::<String>::new());
    }
}
