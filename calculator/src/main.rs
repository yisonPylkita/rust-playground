use std::env;

#[derive(Debug)]
struct RPNCalculator {
    operands: Vec<i32>,
}

impl RPNCalculator {
    fn new() -> RPNCalculator {
        RPNCalculator {
            operands: Vec::new(),
        }
    }

    fn push_operand(&mut self, operand: i32) {
        self.operands.push(operand);
    }

    fn add(&mut self) {
        assert!(self.operands.len() >= 2);
        let operand_a = self.operands.pop().unwrap();
        let operand_b = self.operands.pop().unwrap();
        self.operands.push(operand_a + operand_b);
    }

    fn substract(&mut self) {
        assert!(self.operands.len() >= 2);
        let operand_a = self.operands.pop().unwrap();
        let operand_b = self.operands.pop().unwrap();
        self.operands.push(operand_a - operand_b);
    }

    fn multiply(&mut self) {
        assert!(self.operands.len() >= 2);
        let operand_a = self.operands.pop().unwrap();
        let operand_b = self.operands.pop().unwrap();
        self.operands.push(operand_a * operand_b);
    }

    fn divide(&mut self) {
        assert!(self.operands.len() >= 2);
        let operand_a = self.operands.pop().unwrap();
        let operand_b = self.operands.pop().unwrap();
        assert!(operand_b != 0);
        self.operands.push(operand_a / operand_b);
    }

    fn result(&self) -> i32 {
        assert!(self.operands.len() != 0);
        *(self.operands.last().unwrap())
    }
}

fn main() {
    let arithmetical_expression = match get_arithmetical_expression() {
        Some(i) => i,
        None => {
            print_help();
            panic!("Error: Arithmetical expression not found");
        },
    };

    println!("Arithmetical expression to calculate: '{}'", arithmetical_expression);

    let mut rpn_calc = RPNCalculator::new();

    // Process tokens in arithmetical expression
    for token in arithmetical_expression.split(" ") {
        if token.eq("+") {
            rpn_calc.add();
        } else if token.eq("-") {
            rpn_calc.substract();
        } else if token.eq("*") {
            rpn_calc.multiply();
        } else if token.eq("/") {
            rpn_calc.divide();
        } else {
            match token.parse::<i32>() {
                Ok(operand) => rpn_calc.push_operand(operand),
                Err(error_message) => panic!("Invalid value '{}' in given arithmetical expression - {}", token, error_message),
            }
        }
    }

    println!("Result: {}", rpn_calc.result());
}

fn print_help() {
    println!("calculator");
    println!("Usage: calculator ARITHMETIC_EXPRESSION");
    println!("where");
    println!("ARITHMETIC_EXPRESSION\t arithmetic expression to calculate encoded in RPN");
    println!("");
    println!("Example");
    println!(r#"calculator "2 2 + 3 *""#);
}

fn get_arithmetical_expression() -> Option<String> {
    env::args().nth(1)
}