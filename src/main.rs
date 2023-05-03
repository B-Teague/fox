mod parser;

fn main() {
    let input = "multiply(add(1,2),add(3,4))";
    let result = parser::parse_expr(input);

    match result {
        Ok(expr) => {
            println!("Parsed expression: {:?}", expr);
            // You can now use the parsed expression for further processing
        }
        Err(err) => {
            println!("Parsing error: {:?}", err);
        }
    }
}
