use std::io::Write;

mod tokenize;
mod expr;
mod parse;
mod eval;
mod environment;

fn main() {
    let mut env = environment::Env::new();
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
        match eval::eval_str(input, &mut env) {
            Ok(result) => println!("{}", result.expr_str()),
            Err(err) => println!("Error: {}", err)
        }
    }
}

