mod grammar;
mod ast;
fn main() {
    let res = grammar::parse_program("fun botsbuildbots() { return botsbuildbots(); }");
    println!("res: {:?}", res);
}
