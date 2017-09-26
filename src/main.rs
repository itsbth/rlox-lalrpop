extern crate rustyline;

mod ast;
mod visitor;
mod grammar;
mod resolver;
mod interpreter;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut interp = interpreter::Interpreter::new();
    loop {
        let line = rl.readline("» ");
        match line {
            Ok(line) => {
                rl.add_history_entry(&line);
                let res = grammar::parse_program(line.as_str());
                match res {
                    Ok(ast) => {
                        let mut res = resolver::Resolver::new();
                        let runnable = res.run(ast);
                        println!("[resolved] {:?}", runnable);
                        match interp.run(runnable) {
                            Ok(Some(v)) => println!("« {:?}", v),
                            Ok(None) => (),
                            Err(err) => println!("error: {:?}", err),
                        }
                    }
                    Err(e) => println!("error: {:?}", e),
                }
            }
            _ => break,
        }
    }
}
