use rsb::parser::Parser;

fn negation_normal_form(formula: &str) -> String {
    let mut parser = Parser::new();
    if let Ok(nnf) = parser.evaluate_nnf(formula) {
        return nnf;
    }
    String::from(formula)
}

#[allow(dead_code)]
fn main() {
    negation_normal_form("AB^");
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
    fn exclusive_or() {
        assert_eq!(assert_equal_truthtable("AB^").unwrap(), "AB|A!B!|&");
        assert_eq!(assert_equal_truthtable("A!B^").unwrap(), "A!B|AB!|&");
        assert_eq!(assert_equal_truthtable("AB!^").unwrap(), "AB!|A!B|&");
        assert_eq!(assert_equal_truthtable("A!B!^").unwrap(), "A!B!|AB|&");
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

    #[test]
    fn single_variable() {
        assert_eq!(assert_equal_truthtable("AA=").unwrap(), "A!A|A!A|&");
        assert_eq!(assert_equal_truthtable("A!A=").unwrap(), "AA|A!A!|&");
        assert_eq!(assert_equal_truthtable("AA!=").unwrap(), "A!A!|AA|&");
        assert_eq!(assert_equal_truthtable("A!A!=").unwrap(), "AA!|AA!|&");
    }

    #[test]
    fn eval_sheet() {
        assert_eq!(assert_equal_truthtable("A").unwrap(), "A");
        assert_eq!(assert_equal_truthtable("A!").unwrap(), "A!");
        assert_eq!(assert_equal_truthtable("AB&!").unwrap(), "A!B!|");
        assert_eq!(assert_equal_truthtable("AB|!").unwrap(), "A!B!&");
        assert_eq!(assert_equal_truthtable("AB>!").unwrap(), "AB!&");
        assert_eq!(assert_equal_truthtable("AB=!").unwrap(), "AB!&BA!&|");

        assert_eq!(assert_equal_truthtable("ABC||").unwrap(), "ABC||");
        assert_eq!(assert_equal_truthtable("ABC||!").unwrap(), "A!B!C!&&");
        assert_eq!(assert_equal_truthtable("ABC|&").unwrap(), "ABC|&");
        assert_eq!(assert_equal_truthtable("ABC&|").unwrap(), "ABC&|");
        assert_eq!(assert_equal_truthtable("ABC&|!").unwrap(), "A!B!C!|&");
        assert_eq!(
            assert_equal_truthtable("ABC^^").unwrap(),
            "ABC|B!C!|&|A!B!C!&BC&||&"
        );
        assert_eq!(assert_equal_truthtable("ABC>>").unwrap(), "A!B!C||");
    }
}
