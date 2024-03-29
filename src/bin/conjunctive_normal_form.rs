use rsb::parser::Parser;

fn conjunctive_normal_form(formula: &str) -> String {
    let mut parser = Parser::new();
    if let Ok(cnf) = parser.evaluate_cnf(formula) {
        return cnf;
    }
    String::from(formula)
}

#[allow(dead_code)]
fn main() {
    conjunctive_normal_form("AA&AA&A&");
}

#[cfg(test)]
mod conjunctive_normal_form {
    use super::*;

    fn assert_equal_truthtable(formula: &str) {
        let mut parser = Parser::new();
        if let Ok(table1) = parser.truth_table_from(&formula) {
            if let Ok(cnf) = parser.evaluate_cnf(&formula) {
                if let Ok(table2) = parser.truth_table_from(&cnf) {
                    assert_eq!(table1, table2);
                }
            }
        }
    }

    #[test]
    fn assert_equal() {
        assert_equal_truthtable("AB&!");
        assert_equal_truthtable("AB|!");
        assert_equal_truthtable("AB|C&");
        assert_equal_truthtable("AB|C|D|");
        assert_equal_truthtable("AB&C&D&");
        assert_equal_truthtable("AB&!C!|");
        assert_equal_truthtable("AB|!C!&");
        assert_equal_truthtable("AB&!CD&!>!");
    }

    #[test]
    fn single_variable() {
        assert_equal_truthtable("AA&!");
        assert_equal_truthtable("AA|!");
        assert_equal_truthtable("AA|A&");
        assert_equal_truthtable("AA|A|A|");
        assert_equal_truthtable("AA&AA&A&");
        assert_equal_truthtable("AA&!A!|");
        assert_equal_truthtable("AA|!A!&");
    }

    #[test]
    fn eval_sheet() {
        assert_equal_truthtable("A");
        assert_equal_truthtable("A!");
        assert_equal_truthtable("AB&!");
        assert_equal_truthtable("AB|!");
        assert_equal_truthtable("AB>!");
        assert_equal_truthtable("AB=!");

        assert_equal_truthtable("ABC||");
        assert_equal_truthtable("ABC||!");
        assert_equal_truthtable("ABC|&");
        assert_equal_truthtable("ABC&|");
        assert_equal_truthtable("ABC&|!");
        assert_equal_truthtable("ABC^^");
        assert_equal_truthtable("ABC>>");
    }
}
