use rsb::parser::Parser;

fn conjunction_normal_form(formula: &str) -> String {
    let mut parser = Parser::new();
    if let Ok(nnf) = parser.evaluate_cnf(formula) {
        println!("\n{:?}", nnf);
        return nnf;
    }
    String::from(formula)
}

#[allow(dead_code)]
fn main() {
    conjunction_normal_form("A!");
}

#[cfg(test)]
mod conjunction_normal_form {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(conjunction_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunction_normal_form("AB|!"), "A!B!&");
        assert_eq!(conjunction_normal_form("AB>"), "A!B|");
        assert_eq!(conjunction_normal_form("AB|C&!"), "A!B!&C!|");
    }
}

// ABC|&      AB&AC&|

// AB=     AB&A!B!&|
//         AB>BA>&
//         A!B|B!A|&
