use std::env;

fn get_arithmetical_expression() -> Option<String> {
    env::args().nth(1)
}

fn main() {
    let arithmetical_expression = match get_arithmetical_expression() {
        Some(i) => i,
        None => {
            panic!("Error: Arithmetical expression not found");
        }
    };

    println!("Arithmetical expression to calculate: '{}'", arithmetical_expression);
}
