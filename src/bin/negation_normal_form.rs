use rsb::parser::Parser;

fn negation_normal_form(formula: &str) -> String {
    let mut parser = Parser::new();
    if let Ok(nnf) = parser.evaluate_nnf(formula) {
        println!("\n{:?}", formula);
        println!("\n{:?}", nnf);
        return nnf;
    }
    String::from(formula)
}

#[allow(dead_code)]
fn main() {
    negation_normal_form("A!B&!C|!D!&!E>>>>!&!A>B>!C>!!!F=G!&");
}

#[cfg(test)]
mod negation_normal_form {
    use super::*;

    fn assert_equal_truthtable(formula: &str) -> Option<String> {
        let mut parser = Parser::new();
        if let Ok(table1) = parser.truth_table_from(&formula) {
            if let Ok(nnf) = parser.evaluate_nnf(&formula) {
                if let Ok(table2) = parser.truth_table_from(&nnf) {
                    assert_eq!(table1, table2);
                    return Some(String::from(nnf));
                }
            }
        }
        None
    }

    #[test]
    fn assert_equal() {
        assert_eq!(assert_equal_truthtable("AB&!").unwrap(), "A!B!|");
        assert_eq!(assert_equal_truthtable("AB&!C!&!").unwrap(), "AB&C|");
        assert_eq!(assert_equal_truthtable("A!B&!C&!D!&!").unwrap(), "AB!|C&D|");
        assert_eq!(
            assert_equal_truthtable("A!B&!C|!D!&!E!&!").unwrap(),
            "A!B&C!&D!&E|"
        );
        assert_eq!(assert_equal_truthtable("AB|C&!").unwrap(), "A!B!&C!|");
        assert_eq!(
            assert_equal_truthtable("A!B&!C|!D!&!E!&!A>!!!F=G!&").unwrap(),
            "AB!|C|D|E!&A|F|F!A!B&C!&D!&E|A!&|&G!&"
        );
        assert_eq!(
            assert_equal_truthtable("A!B&!C|!D!&!E!&!A>B>!C>!!!F=G!&").unwrap(),
            "A!B&C!&D!&E|A!&B|C|F|F!AB!|C|D|E!&A|B!&C!&|&G!&"
        );
    }

    #[test]
    fn material_conditions() {
        assert_eq!(assert_equal_truthtable("AB>").unwrap(), "A!B|");
        assert_eq!(assert_equal_truthtable("A!B>").unwrap(), "AB|");
        assert_eq!(assert_equal_truthtable("AB!>").unwrap(), "A!B!|");
        assert_eq!(assert_equal_truthtable("A!B!>").unwrap(), "AB!|");
        assert_eq!(assert_equal_truthtable("AB&!CD&!>!").unwrap(), "A!B!|CD&&");
    }

    #[test]
    fn equivalence() {
        assert_eq!(assert_equal_truthtable("AB=").unwrap(), "A!B|B!A|&");
        assert_eq!(assert_equal_truthtable("A!B=").unwrap(), "AB|B!A!|&");
        assert_eq!(assert_equal_truthtable("AB!=").unwrap(), "A!B!|BA|&");
        assert_eq!(assert_equal_truthtable("A!B!=").unwrap(), "AB!|BA!|&");
    }

    #[test]
    fn morgans_law() {
        assert_eq!(assert_equal_truthtable("AB&!").unwrap(), "A!B!|");
        assert_eq!(assert_equal_truthtable("A!B&!").unwrap(), "AB!|");
        assert_eq!(assert_equal_truthtable("AB!&!").unwrap(), "A!B|");
        assert_eq!(assert_equal_truthtable("A!B!&!").unwrap(), "AB|");
        assert_eq!(assert_equal_truthtable("AB|!").unwrap(), "A!B!&");
        assert_eq!(assert_equal_truthtable("A!B|!").unwrap(), "AB!&");
        assert_eq!(assert_equal_truthtable("AB!|!").unwrap(), "A!B&");
        assert_eq!(assert_equal_truthtable("A!B!|!").unwrap(), "AB&");
    }
}
