use rsb::parser::Parser;

fn negation_normal_form(formula: &str) -> String {
    let mut parser = Parser::new();
    if let Ok(nnf) = parser.evaluate_nnf(formula) {
        println!("\n{:?}", nnf);
        return nnf;
    }
    String::from(formula)
}

#[allow(dead_code)]
fn main() {
    // negation_normal_form("AB|C&!");
    negation_normal_form("XY&!AB|C&!&");
    negation_normal_form("VW&XY&AB|C&!&&!");
    negation_normal_form("AB=");
    negation_normal_form("A!B!=");
    negation_normal_form("A!B!&!");
}

#[cfg(test)]
mod negation_normal_form {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }

    #[test]
    fn material_conditions() {
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("A!B>"), "AB|");
        assert_eq!(negation_normal_form("AB!>"), "A!B!|");
        assert_eq!(negation_normal_form("A!B!>"), "AB!|");
    }

    #[test]
    fn equivalence() {
        assert_eq!(negation_normal_form("AB="), "A!B|B!A|&");
        assert_eq!(negation_normal_form("A!B="), "AB|B!A!|&");
        assert_eq!(negation_normal_form("AB!="), "A!B!|BA|&");
        assert_eq!(negation_normal_form("A!B!="), "AB!|BA!|&");
    }

    #[test]
    fn morgans_law() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("A!B&!"), "AB!|");
        assert_eq!(negation_normal_form("AB!&!"), "A!B|");
        assert_eq!(negation_normal_form("A!B!&!"), "AB|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("A!B|!"), "AB!&");
        assert_eq!(negation_normal_form("AB!|!"), "A!B&");
        assert_eq!(negation_normal_form("A!B!|!"), "AB&");
    }
}

// ABC|&      AB&AC&|

// AB=     AB&A!B!&|
//         AB>BA>&
//         A!B|B!A|&
