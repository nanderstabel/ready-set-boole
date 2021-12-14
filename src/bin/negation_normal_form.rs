use rsb::Parser;

fn negation_normal_form(formula: &str) -> String {
    let mut parser = Parser::new();
    if let Ok(nnf) = parser.negation_normal_form_from(formula) {
        println!("\n{}", nnf);
    }
    String::from(formula)
}

#[allow(dead_code)]
fn main() {
    negation_normal_form("ABCD&|&");
}

#[cfg(test)]
mod negation_normal_form {
    use super::*;

    #[test]
    fn assert_equal() {
        // assert_eq!(negation_normal_form("A!!"), "A");
    }
}
