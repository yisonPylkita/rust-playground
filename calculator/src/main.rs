use std::env;

fn main() {
    let arithmetical_expression = match get_arithmetical_expression() {
        Some(i) => i,
        None => {
            print_help();
            panic!("Error: Arithmetical expression not found");
        }
    };

    println!("Arithmetical expression to calculate: '{}'", arithmetical_expression);
}

fn print_help() {
    println!("calculator");
    println!("Usage: calculator ARITHMETIC_EXPRESSION");
    println!("where");
    println!("ARITHMETIC_EXPRESSION\t arithmetic expression to calculate");
    println!("");
    println!("Example");
    println!(r#"calculator "(2 + 2) * 3""#);
}

fn get_arithmetical_expression() -> Option<String> {
    env::args().nth(1)
}