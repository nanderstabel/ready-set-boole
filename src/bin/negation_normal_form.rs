use rsb::Parser;

fn negation_normal_form(formula: &str) -> String {
    let mut parser = Parser::new();
    if let Ok(nnf) = parser.evaluateNNF(formula) {
        println!("\n{:?}", nnf);
        return nnf;
    }
    String::from(formula)
}

#[allow(dead_code)]
fn main() {
    negation_normal_form("XY&AB|C&!&");
}

#[cfg(test)]
mod negation_normal_form {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }
}

// ABC|&      AB&AC&|

// AB=     AB&A!B!&|
//         AB>BA>&
//         A!B|B!A|&
