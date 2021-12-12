use rsb::Parser;

fn eval_formula(formula: &str) -> bool {
    let mut parser = Parser::new();
    match parser.parse(formula) {
        Ok(_) => {
            println!("{:#?}", parser.tree);
            parser.tree.unwrap().fact
        }
        _ => false,
    }
}

#[allow(dead_code)]
fn main() {
    println!("{}", eval_formula("101|&"));
}

#[cfg(test)]
mod boolean_evaluation {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
        assert_eq!(eval_formula("10|1&"), true);
        assert_eq!(eval_formula("101|&"), true);
    }
}
