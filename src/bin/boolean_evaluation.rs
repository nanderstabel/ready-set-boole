use rsb::parser::Parser;

fn eval_formula(formula: &str) -> bool {
    let mut parser = Parser::new();
    match parser.evaluate(formula) {
        Ok(_) => parser.result.unwrap(),
        _ => false,
    }
}

#[allow(dead_code)]
fn main() {
    eval_formula("");
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
        assert_eq!(eval_formula("1!"), false);
        assert_eq!(eval_formula("0!"), true);
    }

    #[test]
    fn eval_sheet() {
        assert_eq!(eval_formula("0!"), true);
        assert_eq!(eval_formula("1!"), false);
        assert_eq!(eval_formula("00|"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("01|"), true);
        assert_eq!(eval_formula("11|"), true);
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("11&"), true);
        assert_eq!(eval_formula("11^"), false);
        assert_eq!(eval_formula("10^"), true);
        assert_eq!(eval_formula("00>"), true);
        assert_eq!(eval_formula("01>"), true);
        assert_eq!(eval_formula("10>"), false);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("00="), true);
        assert_eq!(eval_formula("11="), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("01="), false);

        assert_eq!(eval_formula("11&0|"), true);
        assert_eq!(eval_formula("10&1|"), true);
        assert_eq!(eval_formula("11&1|"), true);
        assert_eq!(eval_formula("11&1|1^"), false);
        assert_eq!(eval_formula("01&1|1="), true);
        assert_eq!(eval_formula("01&1&1&"), false);
        assert_eq!(eval_formula("0111&&&"), false);
    }
}
